mod lib {
    pub mod add_metadata;
    pub mod config;
}

use lib::add_metadata;
use lib::config::Config;

fn main() {
    let mut config = Config::new();

    config.read_and_validate_filename();

    config.fetch_remaining_args();

    match add_metadata::main(&config) {
        Ok(_) => println!("Successfully generated pdf with meta tags"),
        Err(msg) => println!("Error generating pdf: {}", msg),
    }
}
