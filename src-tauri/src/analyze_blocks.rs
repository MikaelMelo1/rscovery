use std::fs::{self, File};
use std::io::Read;
use tauri::Emitter;

use serde::Serialize;

#[derive(Serialize, Clone)]
struct Progress {
    nonEmpty: Vec<i32>,
    total: f64,
    current: f64,
}


/// Devices like /dev/sdc1 (block devices) are special files, they represent
/// a block device (i.e. a hardware abstraction). The kernel does not store
/// their size in the filesystem metadata. That's why using `metadata().len()` 
/// does not work. 
/// We need to find the number of 512byte sectors to find the size of the device.
fn get_block_device_size_gb(device: &str) -> std::io::Result<f64> {
    let path = format!("/sys/class/block/{}/size", device.replace("/dev/", ""));
    let blocks: u64 = fs::read_to_string(path)?
        .trim()
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let bytes = blocks * 512; 
    let gb = bytes as f64 / 1024.0 / 1024.0;
    Ok(gb) 
}

#[tauri::command]
pub async fn analyze_blocks(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    
    let total_size = get_block_device_size_gb(path).map_err(|e| e.to_string())?;

    let mut buffer = vec![0u8; 32 * 1024 * 1024];
    let mut total_read: u64 = 0;

    let mut iteration = 0;
    let mut non_empty: Vec<i32> = Vec::new();

    loop {
        let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read as u64;
        
        let valid_bytes = buffer.iter().filter(|&&b| b != 0x00).count();
        if valid_bytes > 0 {
            non_empty.push(iteration);
        }
        iteration += 1;

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
