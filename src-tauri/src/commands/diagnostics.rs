use serde::Serialize;
use sysinfo::{MemoryRefreshKind, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessMemoryInfo {
    pub resident_bytes: u64,
    pub virtual_bytes: u64,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
}

#[tauri::command]
pub fn get_process_memory_info() -> Result<ProcessMemoryInfo, String> {
    let pid = sysinfo::Pid::from_u32(std::process::id());
    let process_refresh = ProcessRefreshKind::nothing().with_memory();
    let mut system =
        System::new_with_specifics(RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()))
            .map_err(|error| error.to_string())?;
    system.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), true, process_refresh);
    let process = system.process(pid).ok_or("Current process memory metrics unavailable")?;

    Ok(ProcessMemoryInfo {
        resident_bytes: process.memory(),
        virtual_bytes: process.virtual_memory(),
        total_memory_bytes: system.total_memory(),
        used_memory_bytes: system.used_memory(),
    })
}
