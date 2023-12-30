use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    title: String,
    author: String,
    subject: String,
    keywords: String,
    updated_at: String,
    language: String,
}

impl Config {
    pub fn assign(&mut self, key: &str, value: &str) {
        match key {
            "filename" => self.filename = value.to_string(),
            "title" => self.title = value.to_string(),
            "author" => self.author = value.to_string(),
            "subject" => self.subject = value.to_string(),
            "keywords" => self.keywords = value.to_string(),
            "updated_at" => self.updated_at = value.to_string(),
            "language" => self.language = value.to_string(),
            _ => {
                println!("Unknown key: {}", key);
            }
        }
    }
}

pub fn validate_file(filename: &str) -> Result<(), Box<dyn Error>> {
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

pub fn run(filename: String) {
    println!("You entered: {}", filename)
}
