use std::io;

use meta_morph_pdf::validate_file;
use meta_morph_pdf::Config;

fn main() {
    println!("Enter the filename:");

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    filename = filename.trim().to_string();

    match validate_file(&filename) {
        Ok(_) => println!("Valid file"),
        Err(err) => eprintln!("{}", err),
    }
}
