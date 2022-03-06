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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

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
    println!("{} {}, {}", PKG_NAME, VERSION, AUTHORS);
    let now = Instant::now();
    let args = Args::parse();
    println!("Processing file:{}", &args.file_name);
    let file = fs::File::open(&args.file_name)?;
    let file = BufReader::new(file);

    let (
        thing_count,
        thing_template_count,
        thing_shape_count,
        exported_things,
        exported_thing_templates,
        exported_thing_shapes,
        exported_services,
        exported_subscriptions,
    ) = parse(file, &args.root_path)?;

    println!(
        "Total things:{}, thing templates:{}, thing shapes:{}. ",
        thing_count, thing_template_count, thing_shape_count
    );
    println!(
        "Exported things:{}, thing templates:{}, thing shapes:{}.",
        exported_things, exported_thing_templates, exported_thing_shapes
    );
    println!(
        "Exported services:{}, subscriptions:{}.",
        exported_services, exported_subscriptions
    );
    println!(
        "Successfully exported to folder: {} in {}ms.",
        &args.root_path,
        now.elapsed().as_millis()
    );

    Ok(())
}
