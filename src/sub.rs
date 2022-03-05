use crate::si::*;
use anyhow::Result;
use std::{fs::File, io::BufWriter, io::Write, path::PathBuf};

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
    pub fn export_to_file(&self, path: &PathBuf) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "//{:>20}:\t{}\n", "name", self.name)?;
        write!(writer, "//{:>20}:\t{}\n", "enabled", self.enabled)?;
        write!(writer, "//{:>20}:\t{}\n", "source", self.source)?;
        write!(writer, "//{:>20}:\t{}\n", "event_name", self.event_name)?;
        write!(
            writer,
            "//{:>20}:\t{}\n",
            "source_property", self.source_property
        )?;
        write!(writer, "//{:>20}:\t{}\n", "source_type", self.source_type)?;
        write!(writer, "//{:>20}:\t{}\n", "service_type", self.service_type)?;
        write!(writer, "{}", &self.code)?;
        Ok(())
    }
}
