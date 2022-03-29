mod bindings;
mod parser;
mod servicable;
mod shape;
mod si;
mod sub;
mod template;
mod thing;

use anyhow::{anyhow, Result};

use crate::{bindings::clean_rebuild_binds_csvfile, parser::parse};
use clap::Parser;
use std::{
    fs,
    io::BufReader,
    path::{Path, PathBuf},
    time::Instant,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The source XML file or the root folder of source export to parse
    #[clap(short, long)]
    source_path: String,

    /// The output directory to write the generated files to
    #[clap(short, long, default_value = ".")]
    export_root: String,
}

fn main() -> Result<()> {
    println!("{} {}, {}", PKG_NAME, VERSION, AUTHORS);
    let now = Instant::now();
    let args = Args::parse();
    // println!("Processing file:{}", &args.source_path);
    let filenames = collect_filenames(&args.source_path)?;
    let mut sum_thing_count = 0;
    let mut sum_template_count = 0;
    let mut sum_shape_count = 0;
    let mut sum_exported_things = 0;
    let mut sum_exported_templates = 0;
    let mut sum_exported_shapes = 0;
    let mut sum_exported_services = 0;
    let mut sum_exported_subscriptions = 0;

    clean_rebuild_binds_csvfile(&args.export_root)?;

    for filename in filenames.iter() {
        let file = fs::File::open(filename)?;
        let file = BufReader::new(file);
        // println!("processing file: {:?}", filename);
        let (
            thing_count,
            thing_template_count,
            thing_shape_count,
            exported_things,
            exported_thing_templates,
            exported_thing_shapes,
            exported_services,
            exported_subscriptions,
        ) = match parse(file, &args.export_root) {
            Ok(result) => result,
            Err(err) => {
                println!("Error processing file: {:?}", filename);
                println!("{}", err);
                continue;
            }
        };
        sum_thing_count += thing_count;
        sum_template_count += thing_template_count;
        sum_shape_count += thing_shape_count;
        sum_exported_things += exported_things;
        sum_exported_templates += exported_thing_templates;
        sum_exported_shapes += exported_thing_shapes;
        sum_exported_services += exported_services;
        sum_exported_subscriptions += exported_subscriptions;
    }

    println!(
        "Total things:{}, thing templates:{}, thing shapes:{}. ",
        sum_thing_count, sum_template_count, sum_shape_count
    );
    println!(
        "Exported things:{}, thing templates:{}, thing shapes:{}.",
        sum_exported_things, sum_exported_templates, sum_exported_shapes
    );
    println!(
        "Exported services:{}, subscriptions:{}.",
        sum_exported_services, sum_exported_subscriptions
    );
    println!(
        "Successfully exported to folder: {} in {}ms.",
        &args.export_root,
        now.elapsed().as_millis()
    );

    Ok(())
}

fn collect_filenames(source: &str) -> Result<Vec<PathBuf>> {
    let source_path = Path::new(source);
    let mut filenames = Vec::new();

    if source_path.exists() && source_path.is_file() {
        println!("{} is a file...", source);
        filenames.push(source_path.to_path_buf());
    } else if source_path.exists() && source_path.is_dir() {
        println!("{} is a directory...", source);
        let mut found_things = false;
        let mut found_thing_templates = false;
        let mut found_thing_shapes = false;

        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let pathname = path.to_str().unwrap_or("");
                if pathname.ends_with("Things") || pathname.ends_with("/Things/") {
                    found_things = true;
                } else if pathname.ends_with("ThingTemplates")
                    || pathname.ends_with("/ThingTemplates/")
                {
                    found_thing_templates = true;
                } else if pathname.ends_with("ThingShapes") || pathname.ends_with("/ThingShapes/") {
                    found_thing_shapes = true;
                }
            }
        }

        if !(found_things && found_thing_templates && found_thing_shapes) {
            return Err(anyhow!(
                "{} does not contain a valid Things, ThingTemplates, ThingShapes folder structure",
                source
            ));
        }

        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        let pathname = path.to_str().unwrap_or("");
                        if pathname.ends_with(".xml") {
                            filenames.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
    } else {
        return Err(anyhow!("{} does not exist", source));
    }
    Ok(filenames)
}
