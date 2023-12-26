use font2bytes::FontToBytes;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let src_file_path = args.get(1).cloned();
    let array_name = args.get(2).cloned();

    if let (Some(src_file_path), Some(array_name)) = (src_file_path, array_name) {
        match FontToBytes::new(src_file_path, array_name) {
            Ok(writer) => match writer.run() {
                Ok(output) => println!("{}", output),
                Err(err) => {
                    eprintln!("error: {}", err);
                }
            },
            Err(err) => {
                eprintln!("problem parsing arguments: {}", err);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: cargo run path_to_image_folder name_of_array > filename_to_be_saved.h");
        process::exit(1);
    }
}
