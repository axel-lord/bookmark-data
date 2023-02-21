use bookmark_data::FileData;
use bytesize::ByteSize;
use clap::Parser;
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Cli { file } = Cli::parse();

    let data = FileData::load(BufReader::new(File::open(file)?))?;
    let size = ByteSize::b(data.storage_size().try_into()?);

    println!("size of file contents: {size}");

    Ok(())
}
