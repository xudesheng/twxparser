use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemotePropertyBinding {
    pub entity_name: String,
    pub entity_type: String,
    pub property_name: String,
    pub data_shape: String,
    pub scan_rate: String,
    pub industrial_data_type: String,
    pub start_type: String,
    pub tag_address: String,
    pub tag_type: String,
    pub fold_type: String,
    pub push_type: String,
    pub source_name: String,
    pub timeout: String,
}

pub fn clean_rebuild_binds_csvfile(root: &str) -> Result<()> {
    let path = get_bindings_csvfile(root)?;
    if path.exists() && path.is_file() {
        std::fs::remove_file(&path)?;
    }
    std::fs::create_dir_all(root)?;
    let file = std::fs::File::create(&path)?;
    let mut writer = csv::Writer::from_writer(file);
    writer.write_record(&[
        "entity_name",
        "entity_type",
        "property_name",
        "data_shape",
        "scan_rate",
        "industrial_data_type",
        "start_type",
        "tag_address",
        "tag_type",
        "fold_type",
        "push_type",
        "source_name",
        "timeout",
    ])?;
    // writer.serialize(RemotePropertyBinding::default())?;

    Ok(())
}

pub fn get_bindings_csvfile(root: &str) -> Result<PathBuf> {
    let mut path = PathBuf::from(root);
    path.push("remote_property_bindings.csv");
    Ok(path)
}
