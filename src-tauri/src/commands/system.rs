use crate::models::types::AppError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemProcessInfo {
    pub process_name: String,
    pub display_name: String,
}

fn format_display_name(process_name: &str) -> String {
    let name_without_ext = process_name.replace(".exe", "").replace(".EXE", "");
    
    // Capitalize first letter of each word
    let parts: Vec<&str> = name_without_ext.split(|c| c == '-' || c == '_' || c == ' ').collect();
    let capitalized: Vec<String> = parts.into_iter().map(|p| {
        let mut chars = p.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str()
        }
    }).collect();
    
    capitalized.join(" ")
}

#[tauri::command]
pub async fn get_running_processes() -> Result<Vec<SystemProcessInfo>, AppError> {
    use sysinfo::System;
    let mut sys = System::new();
    
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        sysinfo::ProcessRefreshKind::nothing()
    );
    
    let mut processes = Vec::new();
    for (_pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().into_owned();
        if !name.is_empty() {
            let lower_name = name.to_lowercase();
            
            // Filter common noisy OS background processes to keep picker clean
            if lower_name == "svchost.exe" || lower_name == "conhost.exe" || lower_name == "taskhostw.exe" 
               || lower_name == "system idle process" || lower_name == "system" || lower_name == "registry"
               || lower_name == "smss.exe" || lower_name == "csrss.exe" || lower_name == "wininit.exe"
               || lower_name == "services.exe" || lower_name == "lsass.exe" || lower_name == "winlogon.exe"
               || lower_name == "better-rich-presence.exe" || lower_name == "better_rich_presence.exe"
               || lower_name == "runtimebroker.exe" || lower_name == "ctfmon.exe" || lower_name == "searchhost.exe"
               || lower_name == "shellexperiencehost.exe" || lower_name == "startmenuexperiencehost.exe" {
                continue;
            }
            
            let display_name = format_display_name(&name);
            processes.push(SystemProcessInfo {
                process_name: lower_name,
                display_name,
            });
        }
    }
    
    processes.sort_by(|a, b| a.process_name.cmp(&b.process_name));
    processes.dedup_by(|a, b| a.process_name == b.process_name);
    
    Ok(processes)
}
