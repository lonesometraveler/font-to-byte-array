//! # LCD bitmap
//!
//! Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h
//! Example: cargo run font_13px font_s > font_s.h

use image::Luma;
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

        let mut files = std::fs::read_dir(&Path::new(&folder))?
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
            .collect::<Vec<String>>();

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

        let mut output = self.print_macro(width, height);

        for file in self.files.iter() {
            let path = format!("{}/{}", self.folder, file);
            let arr = match print_array(Path::new(&path)) {
                Ok(f) => f,
                _ => String::from(""),
            };
            output += &arr;
        }

        format!("{}\n}};\n\n#endif", output)
    }

    fn print_macro(&self, width: u32, height: u32) -> String {
        format!(
            "#ifndef {}_H_
#define {}_H_\n
#define {}_IDX_CNT          ({}u)
#define {}_WIDTH_BYTES      ({}u)
#define {}_HEIGHT_ROWS      ({}u)
#define {}_BYTES_PER_CHAR   ({}_HEIGHT_ROWS * {}_WIDTH_BYTES)\n
static const unsigned char {}[{}_IDX_CNT][{}_BYTES_PER_CHAR] = {{",
            self.array_name.to_uppercase(),
            self.array_name.to_uppercase(),
            self.array_name.to_uppercase(),
            self.files.len(),
            self.array_name.to_uppercase(),
            width / 8,
            self.array_name.to_uppercase(),
            height,
            self.array_name.to_uppercase(),
            self.array_name.to_uppercase(),
            self.array_name.to_uppercase(),
            self.array_name,
            self.array_name.to_uppercase(),
            self.array_name.to_uppercase(),
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
            output = format!("{}0x{:02x},", output, byte);
            byte = 0;
        }

        if bit % (12 * 8) == 0 {
            output.push_str("\n\t\t")
        }

        byte = byte.rotate_left(1);
    }

    Ok(format!("{}\n\t}},", output))
}
