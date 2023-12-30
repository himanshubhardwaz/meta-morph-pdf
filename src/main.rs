mod lib {
    pub mod add_meta_tags;
    pub mod config;
}

use lib::add_meta_tags;
use lib::config::Config;

fn main() {
    let mut config = Config::new();

    config.read_and_validate_filename();

    config.fetch_remaining_args();

    add_meta_tags::add_meta_tags(&config);
}
