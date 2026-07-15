//! # Watcher Module
//!
//! Watches for active window changes and idle state using OS-specific APIs.

use crate::models::types::{EngineEvent, AppState};
use log::trace;
use sysinfo::System;
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};

/// Spawns a tokio task that polls for window and idle changes.
pub fn start_window_watcher(tx: Sender<EngineEvent>, app_state: AppState) {
    tauri::async_runtime::spawn(async move {
        let mut sys = System::new_all(); // Need all for full process list
        let mut last_process_name = String::new();
        let mut last_window_title = String::new();
        let mut was_idle = false;

        let mut ticker = interval(Duration::from_secs(2));

        loop {
            ticker.tick().await;

            // 1. Check Idle Status
            #[cfg(target_os = "windows")]
            let (is_idle, idle_minutes) = {
                let idle_ms = win32::get_idle_time_ms();
                let idle_mins = (idle_ms / 1000 / 60) as u32;
                (idle_mins > 0, idle_mins)
            };
            #[cfg(target_os = "linux")]
            let (is_idle, idle_minutes) = {
                let idle_ms = linux::get_idle_time_ms();
                let idle_mins = (idle_ms / 1000 / 60) as u32;
                (idle_mins > 0, idle_mins)
            };
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            let (is_idle, idle_minutes) = (false, 0);

            if is_idle != was_idle {
                let _ = tx.send(EngineEvent::IdleChanged { idle: is_idle, idle_minutes }).await;
                was_idle = is_idle;
            }

            // 2. Check Active Window or Priority App
            let settings = app_state.get_settings().await;
            let rules = app_state.get_app_rules().await;
            let priority_mode = settings.priority_mode_enabled;

            #[cfg(target_os = "windows")]
            let window_info = if priority_mode {
                win32::get_priority_window_info(&mut sys, &rules)
            } else {
                win32::get_foreground_window_info(&mut sys).map(|(p, t)| (p, t, false, None))
            };
            
            #[cfg(target_os = "linux")]
            let window_info = if priority_mode {
                linux::get_priority_window_info(&mut sys, &rules)
            } else {
                linux::get_foreground_window_info(&mut sys).map(|(p, t)| (p, t, false, None))
            };
            
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            let window_info: Option<(String, String, bool, Option<String>)> = None;

            if let Some((process_name, window_title, is_prioritized, foreground_app)) = window_info {
                if process_name != last_process_name || window_title != last_window_title {
                    trace!("Window changed: {} - {}", process_name, window_title);
                    let _ = tx
                        .send(EngineEvent::WindowChanged {
                            process_name: process_name.clone(),
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

#[cfg(target_os = "linux")]
mod linux {
    use sysinfo::System;

    pub fn get_idle_time_ms() -> u32 {
        // Run xprintidle command to get idle time in milliseconds
        if let Ok(output) = std::process::Command::new("xprintidle").output() {
            if let Ok(str_val) = String::from_utf8(output.stdout) {
                if let Ok(ms) = str_val.trim().parse::<u32>() {
                    return ms;
                }
            }
        }
        0
    }

    pub fn get_foreground_window_info(_sys: &mut System) -> Option<(String, String)> {
        // 1. Run xprop to get the active window ID
        let output = std::process::Command::new("xprop")
            .args(&["-root", "_NET_ACTIVE_WINDOW"])
            .output()
            .ok()?;
            
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Output format is: _NET_ACTIVE_WINDOW(WINDOW): window id # 0x420000a
        let id_str = output_str.split("window id #").nth(1)?.trim();
        if id_str.is_empty() || id_str == "0x0" {
            return None;
        }

        // 2. Run xprop to get WM_CLASS and _NET_WM_NAME for this window
        let info_output = std::process::Command::new("xprop")
            .args(&["-id", id_str, "WM_CLASS", "_NET_WM_NAME"])
            .output()
            .ok()?;
            
        let info_str = String::from_utf8_lossy(&info_output.stdout);
        
        let mut wm_class = String::new();
        let mut window_title = String::new();

        for line in info_str.lines() {
            if line.starts_with("WM_CLASS") {
                // WM_CLASS(STRING) = "cursor", "Cursor"
                if let Some(val) = line.split('=').nth(1) {
                    if let Some(last_class) = val.split(',').last() {
                        wm_class = last_class.replace('"', "").trim().to_lowercase();
                    }
                }
            } else if line.starts_with("_NET_WM_NAME") {
                // _NET_WM_NAME(UTF8_STRING) = "filename - folder - Cursor"
                if let Some(val) = line.split('=').nth(1) {
                    window_title = val.replace('"', "").trim().to_string();
                }
            }
        }

        if wm_class.is_empty() {
            return None;
        }

        if wm_class == "better-rich-presence" || wm_class == "better_rich_presence" {
            return None;
        }

        Some((wm_class, window_title))
    }

    pub fn get_priority_window_info(sys: &mut System, _rules: &[crate::models::types::AppRule]) -> Option<(String, String, bool, Option<String>)> {
        // Priority mode not implemented for Linux yet, fallback to foreground
        get_foreground_window_info(sys).map(|(p, t)| (p, t, false, None))
    }
}
