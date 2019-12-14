//! # LCD bitmap 
//! 
//! Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h
//! Example: cargo run font_13px font_13 > font.h

use std::path::Path;
use image::Luma;

pub struct FontToBytes {
    folder: String,
    files: Vec<String>,
    array_name: String,
}

impl FontToBytes {

    pub fn new(mut args: std::env::Args) -> Result<FontToBytes, &'static str> {

        args.next(); // skip the first argument which is the name of the program

        let folder = match args.next() {
            Some(arg) => arg,
            None => return Err("no folder specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h"),
        };

        let array_name = match args.next() {
            Some(arg) => arg,
            None => return Err("no array name specified. Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h")
        };

        let path = Path::new(&folder);
        let paths = std::fs::read_dir(&path).unwrap();

        // let mut files = paths.filter_map(|entry| {
        //     entry.ok().and_then(|e|
        //         e.path().to_str().map(|s| String::from(s))
        //     )
        // }).collect::<Vec<String>>();

        let mut files =
            paths.filter_map(|entry| {
            entry.ok().and_then(|e|
                e.path().file_name()
                .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
            }).collect::<Vec<String>>();

        files.sort();

        Ok(FontToBytes { folder, files, array_name })
    }
}

pub fn run(generator: FontToBytes) -> String {
    // let mut output = String::new();
    let mut output = format!("static const unsigned char {}[][] = {{", generator.array_name); // TODO: return image size and insert array size. add this line at the end with format!

    for file in generator.files {
        let path = format!("{}/{}", generator.folder, file);
        // let arr = match print_array(Path::new(&file)) {
        let arr = match print_array(Path::new(&path)) {
            Ok(f) => f,
            _ => String::from(""),
        };
        output = format!("{}{}", output, arr);
    }
    output = format!("{}\r}};", output);

    output
}

fn print_array(path: &Path) -> Result<String, &'static str> { // TODO: error handling

    let img = image::open(path).unwrap().to_luma();

    // let (width, height) = img.dimensions();

    let mut bit_counter: u32 = 0;
    let mut byte: u8 = 0;

    let mut output = format!("\r\n{{ // {} \r\n", path.to_str().unwrap());

    for pixel in img.pixels() {

        match pixel {
            Luma([0]) => byte &= 0xFE,
            _ => byte |= 0x01,
        }

        if bit_counter % 8 == 7 {
            output = format!("{}0x{:02x},", output, byte);
            byte = 0;
        }

        byte = byte.rotate_left(1);
        bit_counter += 1;
    }

    output = format!("{}\r\n}},", output);
    Ok(output)
}
