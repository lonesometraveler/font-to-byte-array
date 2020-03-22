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
    pub fn new(mut args: std::env::Args) -> Result<FontToBytes, Box<dyn Error>> {
        args.next(); // skip the first argument which is the name of the program

        let folder = match args.next() {
            Some(arg) => arg,
            None => return Err("no folder specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h".into()),
        };

        let array_name = match args.next() {
            Some(arg) => arg,
            None => return Err("no array name specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h".into())
        };

        let mut files: Vec<String> = std::fs::read_dir(&Path::new(&folder))?
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

    pub fn run(&self) -> String {
        let path = format!("{}/{}", self.folder, self.files[0]);
        let (width, height) = image::open(path).unwrap().to_luma().dimensions();

        let macro_defs = self.print_macro(width, height);

        let body = &self
            .files
            .par_iter()
            .filter_map(|file| {
                let path = format!("{}/{}", self.folder, file);
                match print_array(Path::new(&path)) {
                    Ok(f) => Some(f),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            "{macro_defs:}{body:}\n}};\n\n#endif",
            macro_defs = macro_defs,
            body = body
        )
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

fn print_array(path: &Path) -> Result<String, &'static str> {
    let img = image::open(path).unwrap().to_luma();

    let mut byte: u8 = 0;

    let mut output = format!("\n\t{{ // {}", path.to_str().unwrap());

    for (bit, pixel) in img.pixels().enumerate() {
        match pixel {
            Luma([0]) => byte &= 0xFE,
            _ => byte |= 0x01,
        }

        if bit % 8 == 7 {
            output.push_str(&format!("{:#02X},", byte));
            byte = 0;
        }

        if bit % (12 * 8) == 0 {
            output.push_str("\n\t\t")
        }

        byte = byte.rotate_left(1);
    }

    Ok(format!("{}\n\t}},", output))
}
