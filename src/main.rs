use std::env;
use std::process;

use font2bytes::FontToBytes;

fn main() {

    let generator = FontToBytes::new(env::args()).unwrap_or_else( |err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let output = font2bytes::run(generator);
    println!("{}", output);
}
