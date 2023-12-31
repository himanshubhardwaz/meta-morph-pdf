use crate::lib::config::Config;
use printpdf::{Mm, PdfDocument};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub fn add_meta_tags(config: &Config) -> Result<(), Box<dyn Error>> {
    let (doc, _, _) = PdfDocument::new(&config.title, Mm(210.0), Mm(297.0), "Layer 1");

    let file = File::create(&config.export_filename)?;
    let mut buf_writer = BufWriter::new(file);

    doc.with_author(&config.author)
        .with_title(&config.title)
        .with_subject(&config.subject)
        .with_keywords(config.keywords.clone())
        .save(&mut buf_writer)?;

    return Ok(());
}
