//! # Watcher Module
//!
//! Watches for active window changes and idle state using OS-specific APIs.
//! Windows uses Win32 calls (below); Linux delegates to
//! [`crate::services::linux::LinuxDetector`].

use crate::models::types::{AppState, EngineEvent};
#[cfg(not(target_os = "linux"))]
use crate::services::detection_status::{self, DetectionStatus};
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};

/// A detected window: `(process_name, aliases, window_title, is_prioritized, foreground_app)`.
type WindowSnapshot = (String, Vec<String>, String, bool, Option<String>);

/// Spawns a tokio task that polls for window and idle changes.
pub fn start_window_watcher(tx: Sender<EngineEvent>, app_state: AppState) {
    tauri::async_runtime::spawn(async move {
        #[cfg(target_os = "windows")]
        let mut sys = sysinfo::System::new_all(); // Need all for full process list
        #[cfg(target_os = "windows")]
        detection_status::set(DetectionStatus {
            window_backend: Some("win32".to_string()),
            idle_backend: Some("win32".to_string()),
            hint: None,
        });

        #[cfg(target_os = "linux")]
        let mut linux = crate::services::linux::LinuxDetector::new();

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        detection_status::set(DetectionStatus {
            window_backend: None,
            idle_backend: None,
            hint: Some("Activity detection is not supported on this operating system yet.".to_string()),
        });

        let mut last_process_name = String::new();
        let mut last_window_title = String::new();
        // `Some(minutes)` while idle, so every new idle minute is reported.
        let mut last_idle_minutes: Option<u32> = None;

        let mut ticker = interval(Duration::from_secs(2));

        loop {
            ticker.tick().await;

            #[cfg(target_os = "linux")]
            linux.tick();

            // 1. Check Idle Status
            #[cfg(target_os = "windows")]
            let idle_ms = u64::from(win32::get_idle_time_ms());
            #[cfg(target_os = "linux")]
            let idle_ms = linux.idle_ms();
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            let idle_ms: u64 = 0;

            let idle_minutes = u32::try_from(idle_ms / 60_000).unwrap_or(u32::MAX);
            let idle_state = (idle_minutes > 0).then_some(idle_minutes);
            if idle_state != last_idle_minutes {
                let _ = tx
                    .send(EngineEvent::IdleChanged {
                        idle: idle_state.is_some(),
                        idle_minutes,
                    })
                    .await;
                last_idle_minutes = idle_state;
            }

            // 2. Check Active Window or Priority App
            let settings = app_state.get_settings().await;
            let rules = app_state.get_app_rules().await;
            let priority_mode = settings.priority_mode_enabled;

            #[cfg(target_os = "windows")]
            let window_info: Option<WindowSnapshot> = if priority_mode {
                win32::get_priority_window_info(&mut sys, &rules)
                    .map(|(p, t, prioritized, fg)| (p, Vec::new(), t, prioritized, fg))
            } else {
                win32::get_foreground_window_info(&mut sys)
                    .map(|(p, t)| (p, Vec::new(), t, false, None))
            };

            #[cfg(target_os = "linux")]
            let window_info: Option<WindowSnapshot> = if priority_mode {
                linux.priority_window(&rules)
            } else {
                linux.foreground_window(&rules).map(|w| (w, false, None))
            }
            .map(|(w, prioritized, fg)| (w.process_name, w.aliases, w.title, prioritized, fg));

            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            let window_info: Option<WindowSnapshot> = {
                let _ = (priority_mode, &rules);
                None
            };

            if let Some((process_name, aliases, window_title, is_prioritized, foreground_app)) = window_info {
                if process_name != last_process_name || window_title != last_window_title {
                    trace!("Window changed: {} - {}", process_name, window_title);
                    let _ = tx
                        .send(EngineEvent::WindowChanged {
                            process_name: process_name.clone(),
                            aliases,
                            window_title: window_title.clone(),
                            is_prioritized,
                            foreground_app,
                        })
                        .await;

                    last_process_name = process_name;
                    last_window_title = window_title;
                }
            } else if !last_process_name.is_empty() {
                let _ = tx
                    .send(EngineEvent::WindowChanged {
                        process_name: String::new(),
                        aliases: Vec::new(),
                        window_title: String::new(),
                        is_prioritized: false,
                        foreground_app: None,
                    })
                    .await;
                last_process_name = String::new();
                last_window_title = String::new();
            }
        }
    });
}

#[cfg(target_os = "windows")]
mod win32 {
    use sysinfo::System;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
        IsWindowVisible,
    };
    use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
    use crate::models::types::AppRule;

    pub fn get_idle_time_ms() -> u32 {
        let mut last_input = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        unsafe {
            if GetLastInputInfo(&mut last_input).as_bool() {
                let tick_count = windows::Win32::System::SystemInformation::GetTickCount();
                return tick_count.saturating_sub(last_input.dwTime);
            }
        }
        0
    }

    pub fn get_foreground_window_info(sys: &mut System) -> Option<(String, String)> {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null() {
                return None;
            }

            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            if pid == 0 {
                return None;
            }

            let mut title_buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut title_buf);
            let window_title = String::from_utf16_lossy(&title_buf[..len as usize]);

            // Refresh only the specific process to save CPU
            let pid_val = sysinfo::Pid::from_u32(pid);
            sys.refresh_processes_specifics(
                sysinfo::ProcessesToUpdate::Some(&[pid_val]),
                true,
                sysinfo::ProcessRefreshKind::everything()
                    .without_cpu()
                    .without_memory()
                    .without_disk_usage(),
            );

            let process_name = sys
                .process(sysinfo::Pid::from_u32(pid))
                .map(|p| p.name().to_string_lossy().into_owned())
                .unwrap_or_else(|| format!("unknown_{}.exe", pid));
                
            // sysinfo often returns uppercase for some process names, standardize it to lowercase for matching
            let process_name = process_name.to_lowercase();

            if process_name == "better-rich-presence.exe" || process_name == "better_rich_presence.exe" {
                return None;
            }

            Some((process_name, window_title))
        }
    }

    struct EnumData {
        target_pid: u32,
        found_title: Option<String>,
    }

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut EnumData);

        if IsWindowVisible(hwnd).as_bool() {
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));

            if pid == data.target_pid {
                let mut title_buf = [0u16; 512];
                let len = GetWindowTextW(hwnd, &mut title_buf);
                if len > 0 {
                    let title = String::from_utf16_lossy(&title_buf[..len as usize]);
                    if !title.is_empty() {
                        data.found_title = Some(title);
                        return BOOL(0); // Stop enumerating
                    }
                }
            }
        }
        BOOL(1) // Continue
    }

    pub fn get_priority_window_info(sys: &mut System, rules: &[AppRule]) -> Option<(String, String, bool, Option<String>)> {
        sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        let mut highest_priority_idx = usize::MAX;
        let mut selected_pid = None;
        let mut selected_process_name = String::new();

        for (pid, process) in sys.processes() {
            let p_name = process.name().to_string_lossy().to_lowercase();
            if p_name == "better-rich-presence.exe" || p_name == "better_rich_presence.exe" {
                continue;
            }

            if let Some(idx) = rules.iter().position(|r| r.enabled && r.process_name.to_lowercase().trim_end_matches(".exe") == p_name.trim_end_matches(".exe")) {
                if idx < highest_priority_idx {
                    highest_priority_idx = idx;
                    selected_pid = Some(pid.as_u32());
                    selected_process_name = p_name.clone();
                }
            }
        }

        let foreground = get_foreground_window_info(sys);
        
        if let Some(target_pid) = selected_pid {
            let mut data = EnumData {
                target_pid,
                found_title: None,
            };

            unsafe {
                let _ = EnumWindows(
                    Some(enum_proc),
                    LPARAM(&mut data as *mut EnumData as isize),
                );
            }

            let final_title = data.found_title.unwrap_or_else(|| selected_process_name.clone());

            let mut fg_app = None;
            let mut is_prioritized = false;

            if let Some((fg_name, _)) = &foreground {
                if fg_name.to_lowercase().trim_end_matches(".exe") != selected_process_name.trim_end_matches(".exe") {
                    is_prioritized = true;
                    fg_app = Some(fg_name.clone());
                }
            }

            return Some((selected_process_name, final_title, is_prioritized, fg_app));
        }

        // Fallback to foreground if no matching priority app
        foreground.map(|(p, t)| (p, t, false, None))
    }
}
