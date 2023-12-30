use std::io;

use meta_morph_pdf::Config;

fn main() {
    println!("Enter the filename:");

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    filename = filename.trim().to_string();

    meta_morph_pdf::run(filename);
}
