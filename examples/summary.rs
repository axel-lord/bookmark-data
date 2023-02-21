use anyhow::Context;
use bookmark_data::FileData;
use clap::Parser;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Cli { file } = Cli::parse();

    let data = FileData::load(BufReader::new(
        File::open(file).with_context(|| "failed to open file")?,
    ))
    .with_context(|| "failed to read file contents to a FileData struct")?;

    println!("{} categories", data.category.len());
    println!("{} Bookmarks", data.bookmark.len());

    if let Some(bookmark) = data.bookmark.first() {
        println!("- first bookmark -\n{bookmark:#?}")
    }

    Ok(())
}
