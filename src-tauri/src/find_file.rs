// User sends the valid blocks indexes (32MB each)
// We need to return the absolute position (in HEX) of the specific 
// magic byte. 

// For now, the user does not specify custom magic bytes, but needs to
// select from our list.

// Ex: finds the start position of a valid block (e.g. valid image).

use std::io::Read;
use std::sync::OnceLock;
use std::{
    collections::HashSet,
    fs::{self, File},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use sha256::digest;

static SIGNATURES: OnceLock<Vec<MagicByte>> = OnceLock::new();

/// All default signatures
pub fn get_signatures() -> &'static Vec<MagicByte<'static>> {
    SIGNATURES.get_or_init(|| {
        vec![
            MagicByte::new(
                &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
                &[0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82],
                "png",
                200 * 1024 * 1024,
                "PNG",
                true,
            ),
            MagicByte::new(
                &[0xFF, 0xD8],
                &[0xFF, 0xD9],
                "jpeg",
                500 * 1024 * 1024,
                "JPEG",
                true,
            ),
            MagicByte::new(
                &[0x50, 0x4B, 0x03, 0x04],
                &[0x50, 0x4B, 0x05, 0x06],
                "zip",
                500 * 1024 * 1024,
                "ZIP",
                false,
            ),
            MagicByte::new(
                &[0x25, 0x50, 0x44, 0x46, 0x2D],
                &[0x25, 0x25, 0x45, 0x4F, 0x46],
                "pdf",
                500 * 1024 * 1024,
                "PDF",
                false,
            ),
        ]
    })
}

pub struct MagicByte<'s> {
    signature: &'s [u8],
    end: &'s [u8],
    extension: &'s str,
    max_size: usize,
    pub name: &'s str,
    is_image: bool,
}

impl<'s> MagicByte<'s> {
    pub fn new(
        signature: &'s [u8],
        end: &'s [u8],
        extension: &'s str,
        max_size: usize,
        name: &'s str,
        is_image: bool,
    ) -> MagicByte<'s> {
        MagicByte {
            signature,
            end,
            extension,
            max_size,
            name,
            is_image,
        }
    }
}

impl<'s> MagicByte<'s> {
    pub fn extract(&self, running: &Arc<AtomicBool>, file: &mut File) -> std::io::Result<()> {
        let mut buffer = vec![0u8; 16 * 1024 * 1024];
        let mut total_read: u64 = 0;

        let mut file_buffer: Vec<u8> = Vec::new();
        let mut file_hash: HashSet<String> = HashSet::new();

        let mut searching_file = false;
        let mut sig_match_index = 0;

        let mut count = 0;

        loop {
            if !running.load(Ordering::SeqCst) {
                println!("Ending IO...");
                break;
            }

            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            total_read += bytes_read as u64;

            for &b in buffer[..bytes_read].iter() {
                if searching_file {
                    file_buffer.push(b);

                    if file_buffer.len() > self.max_size {
                        println!("{} excedeu tamanho máximo", self.name);
                        searching_file = false;
                        file_buffer.clear();
                    } else if file_buffer.len() >= self.end.len()
                        && file_buffer[file_buffer.len() - self.end.len()..] == *self.end
                    {
                        // @TODO: This will fail in other file extensions
                        // like ZIP or PDF, that are not a "image"
                        if self.is_image {
                            if image::load_from_memory(&file_buffer).is_ok() {
                                let hash = digest(&file_buffer);
                                if file_hash.insert(hash.clone()) {
                                    let filename =
                                        format!("{}_{count}.{}", self.extension, self.extension);
                                    fs::write(&filename, &file_buffer)
                                        .expect("Falha ao salvar PNG");
                                    println!("Saved {} ({} bytes)", filename, file_buffer.len());
                                    count += 1;
                                }
                            }
                        } else {
                            let filename = format!("{}_{count}.{}", self.extension, self.extension);
                            fs::write(&filename, &file_buffer).expect("Falha ao salvar PNG");
                            println!("Saved {} ({} bytes)", filename, file_buffer.len());
                            count += 1;
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

            let progress_mb = total_read as f64 / 1024.0 / 1024.0;
            println!("Progress: {:.2} MB", progress_mb);
        }

        println!("Program ended successfully.");
        Ok(())
    }
}
