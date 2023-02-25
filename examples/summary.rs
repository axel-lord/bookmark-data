use anyhow::Context;
use bookmark_data::file;
use clap::Parser;
use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    tree: bool,
    #[arg(short, long, default_value_t = false)]
    category_sample: bool,
    #[arg(short, long, default_value_t = false)]
    bookmark_sample: bool,
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let Cli {
        file,
        tree,
        category_sample,
        bookmark_sample,
    } = Cli::parse();

    let t = Instant::now();
    let data = file::File::load(BufReader::new(
        File::open(file).with_context(|| "failed to open file")?,
    ))
    .with_context(|| "failed to read file contents to a FileData struct")?;
    let t = t.elapsed();

    if tree {
        println!("- category tree -");
        println!("{:#?}", data.category)
    }

    if bookmark_sample {
        if let Some(bookmark) = data.bookmark.first() {
            println!("- first bookmark -");
            println!("{bookmark:#?}")
        }

        if let Some(bookmark) = data.bookmark.last() {
            println!("- last bookmark -");
            println!("{bookmark:#?}")
        }
    }

    if category_sample {
        if let Some(category) = data.category.first() {
            println!("- first category -");
            println!("{category:#?}")
        }

        if let Some(category) = data.category.last() {
            println!("- last category -");
            println!("{category:#?}")
        }
    }

    println!("{} categories", data.category.len());
    println!("{} Bookmarks", data.bookmark.len());
    println!("{}ms parse time", t.as_secs_f64() * 1000.0);

    Ok(())
}
