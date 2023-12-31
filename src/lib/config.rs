use std::error::Error;
use std::fs;
use std::io;
use std::process;

#[derive(Debug, Default)]
pub struct Config {
    pub filename: String,
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: Vec<String>,
    pub export_filename: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config: Config = Default::default();
        config.export_filename = "custom_export_filename".to_string();
        config
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

        let mut keywords_input = String::new();

        io::stdin()
            .read_line(&mut keywords_input)
            .expect("Failed to read line");

        self.keywords = keywords_input
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

        println!("Enter new pdf file name: ");

        io::stdin()
            .read_line(&mut self.export_filename)
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
