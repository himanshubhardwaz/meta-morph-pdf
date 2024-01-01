use crate::lib::config::Config;
use lopdf::{Dictionary, Document, Object};
use std::error::Error;

pub fn main(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut doc = Document::load(&config.filename)?;

    let metadata = Object::Dictionary({
        let mut dict = Dictionary::new();
        dict.set(
            "Title".to_string(),
            Object::string_literal(config.title.as_str()),
        );
        dict.set(
            "Author".to_string(),
            Object::string_literal(config.author.as_str()),
        );
        dict.set(
            "Subject".to_string(),
            Object::string_literal(config.subject.as_str()),
        );
        dict.set(
            "Keywords".to_string(),
            Object::string_literal(config.keywords.as_str()),
        );
        dict
    });

    let metadata_id = doc.add_object(metadata);

    doc.trailer.set("Info", metadata_id);

    doc.save(&config.export_filename)?;

    Ok(())
}
