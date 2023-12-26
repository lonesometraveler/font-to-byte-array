//! # LCD bitmap
//!
//! Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h
//! Example: cargo run font_13px font_s > font_s.h

use image::Luma;
use rayon::prelude::*;
use std::error::Error;
use std::path::Path;

pub struct FontToBytes {
    folder: String,
    files: Vec<String>,
    array_name: String,
}

impl FontToBytes {
    pub fn new(folder: String, array_name: String) -> Result<FontToBytes, Box<dyn Error>> {
        let mut files: Vec<String> = std::fs::read_dir(Path::new(&folder))?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    match e.path().extension().and_then(std::ffi::OsStr::to_str) {
                        Some("png") => e
                            .path()
                            .file_name()
                            .and_then(|name| name.to_str().map(String::from)),
                        _ => None,
                    }
                })
            })
            .collect();

        files.sort();

        Ok(FontToBytes {
            folder,
            files,
            array_name,
        })
    }

    pub fn run(&self) -> Result<String, Box<dyn Error>> {
        let path = format!("{}/{}", self.folder, self.files[0]);
        let (width, height) = image::open(path)?.to_luma8().dimensions();

        let macro_defs = self.print_macro(width, height);

        let body = self
            .files
            .par_iter()
            .filter_map(|file| {
                let path = format!("{}/{}", self.folder, file);
                print_array(&path).ok()
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(format!("{macro_defs}{body}\n}};\n\n#endif"))
    }

    fn print_macro(&self, width: u32, height: u32) -> String {
        format!(
            "#ifndef {0}_H_
#define {0}_H_\n
#define {0}_IDX_CNT          ({1}u)
#define {0}_WIDTH_BYTES      ({2}u)
#define {0}_HEIGHT_ROWS      ({3}u)
#define {0}_BYTES_PER_CHAR   ({0}_HEIGHT_ROWS * {0}_WIDTH_BYTES)\n
static const unsigned char {4}[{0}_IDX_CNT][{0}_BYTES_PER_CHAR] = {{",
            self.array_name.to_uppercase(),
            self.files.len(),
            width / 8,
            height,
            self.array_name,
        )
    }
}

fn print_array(file_path: &str) -> Result<String, Box<dyn Error>> {
    // Open the image
    let img = image::open(file_path)?.to_luma8();

    let mut byte: u8 = 0;
    let mut bytes: Vec<u8> = Vec::new();

    let mut output = format!("\n\t// {file_path}\n\t{{");

    // Iterate over the pixels
    for (bit, pixel) in img.pixels().enumerate() {
        match pixel {
            // clear LSB
            Luma([0]) => byte &= 0xFE,
            // set LSB
            _ => byte |= 0x01,
        }

        // Store the byte after accumulating 8 bits
        if bit % 8 == 7 {
            bytes.push(byte);
            byte = 0;
        }

        // Rotate the byte to the left
        byte = byte.rotate_left(1);
    }

    // Insert commas and a new line after printing every 12 bytes
    let joined_bytes = format_bytes(bytes, 12);
    output.push_str(&joined_bytes);

    // Close the array
    output.push_str("},");

    Ok(output)
}

// Format bytes for C style array
fn format_bytes(data: Vec<u8>, items_per_line: usize) -> String {
    data.chunks(items_per_line)
        .map(|chunk| {
            let line: Vec<String> = chunk.iter().map(|item| format!("0x{:02X}", item)).collect();
            line.join(", ")
        })
        .collect::<Vec<String>>()
        .join(",\n\t ")
}
