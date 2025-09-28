use serde::Serialize;
use sysinfo::Disks;
use tauri::{Manager, Builder};

mod analyze_blocks;

#[derive(Debug, Serialize)]
pub struct DiskInfo {
    name: String,
    size: u64,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_disks() -> Vec<DiskInfo> {
    let mut disks = Disks::new_with_refreshed_list();
    disks.refresh_list();

    disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.mount_point().to_string_lossy().to_string(),
            size: disk.total_space() / 1024 / 1024,
        })
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_disks,
            analyze_blocks::analyze_blocks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
