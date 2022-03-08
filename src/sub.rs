use crate::si::*;
use anyhow::Result;
use std::{fs::File, io::BufWriter, io::Write, path::Path};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subscription {
    pub enabled: bool,
    pub event_name: String,
    pub name: String,
    pub source: String,
    pub source_property: String,
    pub source_type: String,
    pub code: String,
    pub service_type: ServiceHandler,
}

impl Subscription {
    pub fn export_to_file(&self, path: &Path, leading_prefix: &str) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}{:>20}:\t{}", leading_prefix, "name", self.name)?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "enabled", self.enabled
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "source", self.source
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "event_name", self.event_name
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "source_property", self.source_property
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "source_type", self.source_type
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "service_type", self.service_type
        )?;
        write!(writer, "{}", &self.code)?;
        Ok(())
    }
}
