pub struct Config {
    pub filename: String,
    title: String,
    author: String,
    subject: String,
    keywords: String,
    updated_at: String,
    language: String,
}

pub fn run(filename: String) {
    println!("You entered: {}", filename)
}
