use std::env;
use std::process;

use font2bytes::FontToBytes;

fn main() {
    let writer = FontToBytes::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", writer.run());
}
