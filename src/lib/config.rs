use std::error::Error;
use std::fs;
use std::io;
use std::process;

#[derive(Debug)]
pub struct Config {
    filename: String,
    title: String,
    author: String,
    subject: String,
    keywords: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            filename: String::new(),
            title: String::new(),
            author: String::new(),
            subject: String::new(),
            keywords: String::new(),
        }
    }

    pub fn read_and_validate_filename(&mut self) {
        println!("Enter the filename: ");

        io::stdin()
            .read_line(&mut self.filename)
            .expect("Failed to read line");

        self.filename = self.filename.trim().to_string();

        match validate_file(&self.filename) {
            Ok(_) => println!("Valid file"),
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    }

    pub fn fetch_remaining_args(&mut self) {
        println!("Enter title: ");
        io::stdin()
            .read_line(&mut self.title)
            .expect("Failed to read line");

        println!("Enter author name: ");
        io::stdin()
            .read_line(&mut self.author)
            .expect("Failed to read line");

        println!("Enter subject: ");
        io::stdin()
            .read_line(&mut self.subject)
            .expect("Failed to read line");

        println!("Enter keywords seperated by commas(,): ");

        io::stdin()
            .read_line(&mut self.keywords)
            .expect("Failed to read line");
    }
}

fn validate_file(filename: &str) -> Result<(), Box<dyn Error>> {
    if let Some(extension) = filename.rsplit('.').next() {
        if extension.to_lowercase() == "pdf" {
            let metadata = fs::metadata(filename)?;
            if metadata.is_file() {
                return Ok(());
            } else {
                return Err(format!("Error: '{}' is not a file.", filename).into());
            }
        }
    }
    Err(format!("Error: '{}' is not a valid PDF file.", filename).into())
}
