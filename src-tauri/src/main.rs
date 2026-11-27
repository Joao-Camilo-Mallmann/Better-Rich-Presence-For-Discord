// Prevents additional console window on Windows across all build profiles, DO NOT REMOVE!!
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

fn main() {
    better_rich_presence_lib::run()
}
