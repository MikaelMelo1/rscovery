// User sends the valid blocks indexes (32MB each)
// We need to return the absolute position (in HEX) of the specific
// magic byte.

// For now, the user does not specify custom magic bytes, but needs to
// select from our list.

use std::io::Read;
use std::{
    collections::HashSet,
    fs::{self, File},
};

use base64::Engine;
use serde::Serialize;
use sha256::digest;
use tauri::Emitter;

use crate::analyze_blocks::get_block_device_size_gb;

/// All default signatures
/// When `is_image`, it'll send the image as b64 to the frontend, and
/// the file will not be saved in the disk. Otherwhise, it'll save into the disk
/// and return the path to the frontend.
pub struct MagicByte<'s> {
    signature: &'s [u8],
    end: &'s [u8],
    extension: &'s str,
    max_size: usize,
    pub name: &'s str,
    is_image: bool,
}

#[derive(Serialize, Clone)]
struct ImageFound {
    base64: String,
}

#[derive(Serialize, Clone)]
struct FileFind {
    path: String,
    size: f64,
}

#[derive(Serialize, Clone)]
struct Progress {
    current: f64,
    total: f64,
}

#[tauri::command]
pub async fn find_jpeg(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    MagicByte {
        signature: &[0xFF, 0xD8],
        end: &[0xFF, 0xD9],
        extension: "jpeg",
        max_size: 500 * 1024 * 1024,
        name: "JPEG",
        is_image: true,
    }
    .extract(app_handle, path, 300)
}

#[tauri::command]
pub async fn find_png(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    MagicByte {
        signature: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
        end: &[0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82],
        extension: "png",
        max_size: 200 * 1024 * 1024,
        name: "PNG",
        is_image: true,
    }
    .extract(app_handle, path, 300)
}

#[tauri::command]
pub async fn find_pdf(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    MagicByte {
        signature: &[0x25, 0x50, 0x44, 0x46, 0x2D],
        end: &[0x25, 0x25, 0x45, 0x4F, 0x46],
        extension: "pdf",
        max_size: 500 * 1024 * 1024,
        name: "PDF",
        is_image: false,
    }
    .extract(app_handle, path, 300)
}

#[tauri::command]
pub async fn find_zip(app_handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    MagicByte {
        signature: &[0x50, 0x4B, 0x03, 0x04],
        end: &[0x50, 0x4B, 0x05, 0x06],
        extension: "zip",
        max_size: 500 * 1024 * 1024,
        name: "ZIP",
        is_image: false,
    }
    .extract(app_handle, path, 300)
}

impl<'s> MagicByte<'s> {
    pub fn extract(
        &self,
        app_handle: tauri::AppHandle,
        path: &str,
        max: i32,
    ) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;

        let total_size = get_block_device_size_gb(path).map_err(|e| e.to_string())?;

        let mut buffer = vec![0u8; 32 * 1024 * 1024];
        let mut total_read: u64 = 0;

        let mut file_buffer: Vec<u8> = Vec::new();
        let mut file_hash: HashSet<String> = HashSet::new();

        let mut searching_file = false;
        let mut sig_match_index = 0;

        let mut count = 0;

        app_handle
            .emit(
                "file-progress",
                Progress {
                    current: 0.0,
                    total: total_size,
                },
            )
            .unwrap();

        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| e.to_string())?;
            if bytes_read == 0 {
                break;
            }
            total_read += bytes_read as u64;

            if count >= max {
                break;
            }

            for &b in buffer[..bytes_read].iter() {
                if searching_file {
                    file_buffer.push(b);

                    if file_buffer.len() > self.max_size {
                        searching_file = false;
                        file_buffer.clear();
                        continue;
                    }
                    if file_buffer.len() >= self.end.len()
                        && file_buffer[file_buffer.len() - self.end.len()..] == *self.end
                    {
                        if self.is_image {
                            if image::load_from_memory(&file_buffer).is_ok() {
                                let hash = digest(&file_buffer);
                                if file_hash.insert(hash.clone()) {
                                    let base64 = base64::engine::general_purpose::STANDARD
                                        .encode(&file_buffer);
                                    count += 1;
                                    app_handle
                                        .emit("file-found", ImageFound { base64 })
                                        .unwrap();
                                }
                            }
                        } else {
                            let hash = digest(&file_buffer);
                            if file_hash.insert(hash.clone()) {
                                let filename = format!(
                                    "../found/{}_{count}.{}",
                                    self.extension, self.extension
                                );
                                fs::write(&filename, &file_buffer)
                                    .expect("Error while saving file");

                                app_handle
                                    .emit(
                                        "file-found",
                                        FileFind {
                                            path: filename,
                                            size: file_buffer.len() as f64 / 1024.0,
                                        },
                                    )
                                    .unwrap();

                                count += 1;
                            }
                        }

                        searching_file = false;
                        file_buffer.clear();
                        sig_match_index = 0;
                    }
                    continue;
                }

                if b == self.signature[sig_match_index] {
                    sig_match_index += 1;
                    if sig_match_index == self.signature.len() {
                        searching_file = true;
                        file_buffer.clear();
                        file_buffer.extend_from_slice(self.signature);
                        sig_match_index = 0;
                    }
                } else {
                    sig_match_index = 0;
                }
            }

            app_handle
                .emit(
                    "file-progress",
                    Progress {
                        current: total_read as f64 / 1024.0 / 1024.0,
                        total: total_size,
                    },
                )
                .unwrap();
        }

        Ok(())
    }
}
