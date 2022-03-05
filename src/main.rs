mod parser;
mod servicable;
mod shape;
mod si;
mod sub;
mod template;
mod thing;

use anyhow::Result;

use crate::parser::parse;
use clap::Parser;
use std::{fs, io::BufReader, time::Instant};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The source XML file to parse
    #[clap(short, long)]
    file_name: String,

    /// The output directory to write the generated files to
    #[clap(short, long, default_value = ".")]
    root_path: String,
}

fn main() -> Result<()> {
    let now = Instant::now();
    let args = Args::parse();
    println!("reading file:{}", &args.file_name);
    let file = fs::File::open(&args.file_name)?;
    let file = BufReader::new(file);

    let (thing_count, thing_template_count, thing_shape_count) = parse(file, &args.root_path)?;

    println!(
        "total things:{}, thing templates:{}, thing shapes:{}",
        thing_count, thing_template_count, thing_shape_count
    );
    println!(
        "Successfully exported to folder:{} in {}s",
        &args.root_path,
        now.elapsed().as_secs()
    );

    Ok(())
}
