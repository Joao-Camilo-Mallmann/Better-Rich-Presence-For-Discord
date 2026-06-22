//! # Watcher Module
//!
//! Watches for active window changes and idle state using Win32 API.

use crate::types::EngineEvent;
use log::trace;
use sysinfo::System;
use tokio::sync::mpsc::Sender;
use tokio::time::{interval, Duration};

/// Spawns a tokio task that polls for window and idle changes.
pub fn start_window_watcher(tx: Sender<EngineEvent>) {
    tauri::async_runtime::spawn(async move {
        let mut sys = System::new();
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
                // Idle check is handled downstream by engine settings,
                // but we emit idle events so the engine can decide based on threshold.
                (idle_mins > 0, idle_mins)
            };
            #[cfg(not(target_os = "windows"))]
            let (is_idle, idle_minutes) = (false, 0);

            if is_idle != was_idle {
                let _ = tx.send(EngineEvent::IdleChanged { idle: is_idle, idle_minutes }).await;
                was_idle = is_idle;
            }

            // 2. Check Active Window
            #[cfg(target_os = "windows")]
            let window_info = win32::get_foreground_window_info(&mut sys);
            #[cfg(not(target_os = "windows"))]
            let window_info = None;

            if let Some((process_name, window_title)) = window_info {
                if process_name != last_process_name || window_title != last_window_title {
                    trace!("Window changed: {} - {}", process_name, window_title);
                    let _ = tx
                        .send(EngineEvent::WindowChanged {
                            process_name: process_name.clone(),
                            window_title: window_title.clone(),
                        })
                        .await;

                    last_process_name = process_name;
                    last_window_title = window_title;
                }
            }
        }
    });
}

#[cfg(target_os = "windows")]
mod win32 {
    use sysinfo::System;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };

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
}
