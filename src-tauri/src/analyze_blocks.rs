use std::fs::{self, File};
use std::io::Read;
use tauri::Emitter;

use serde::Serialize;

#[derive(Serialize, Clone)]
struct Progress {
    nonEmpty: Vec<i32>,
    total: u64,
    current: f64,
}

#[tauri::command]
pub fn analyze_blocks(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    // let mut file =
    //     File::open(format!(r"\\.\{}", path.trim_end_matches('\\'))).map_err(|e| e.to_string())?;
    let mut file = File::open(r"\\.\F:").map_err(|e| e.to_string())?;
    let total_size = file.metadata().map_err(|e| e.to_string())?.len() / 1024 / 1024;

    let mut buffer = vec![0u8; 32 * 1024 * 1024];
    let mut total_read: u64 = 0;

    let mut iteration = 0;
    let mut non_empty: Vec<i32> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
        iteration += 1;
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read as u64;

        let valid_bytes = buffer.iter().filter(|&&b| b != 0x00).count();
        if valid_bytes > 0 {
            non_empty.push(iteration);
        }

        let progress = Progress {
            nonEmpty: non_empty.clone(),
            total: total_size,
            current: total_read as f64 / 1024.0 / 1024.0,
        };

        println!("Progresso: {:.2} MB", &progress.current);
        app_handle.emit("scan-progress", progress).unwrap();
    }

    println!("Reading completed.");
    Ok(())
}
