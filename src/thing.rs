use crate::si::*;
use anyhow::Result;
use std::path::PathBuf;

pub const KEY_NAME: &str = "things";
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thing {
    pub name: String,
    pub template: String,
    pub services: Vec<Service>,
}

impl Thing {
    pub fn clean_folder(&self, root: &str) -> Result<()> {
        let mut path = PathBuf::from(root);
        path.push(KEY_NAME);
        path.push(&self.name);

        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(())
    }
    pub fn build_folder(&self, root: &str) -> Result<()> {
        let mut path = PathBuf::from(root);
        path.push(KEY_NAME);
        path.push(&self.name);

        if path.exists() && path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;

        Ok(())
    }

    pub fn export_services(&self, root: &str) -> Result<()> {
        self.clean_folder(root)?;
        if self.services.len() == 0 {
            return Ok(());
        }
        self.build_folder(root)?;

        let mut path = PathBuf::from(root);
        path.push(KEY_NAME);
        path.push(&self.name);

        for service in &self.services {
            let mut service_path = path.clone();
            service_path.push(&service.name);
            match service.service_type {
                ServiceHandler::Scrit => {
                    service_path.set_extension("js");
                    std::fs::write(&service_path, &service.code)?;
                }
                ServiceHandler::SQLQuery => {
                    service_path.set_extension("sql");
                    std::fs::write(&service_path, &service.code)?;
                }
                ServiceHandler::SQLCommand => {
                    service_path.set_extension("sql");
                    std::fs::write(&service_path, &service.code)?;
                }
                ServiceHandler::Route => {
                    service_path.set_extension("json");
                    std::fs::write(&service_path, &service.code)?;
                }
                ServiceHandler::Reflection => {
                    service_path.set_extension("Reflection");
                    std::fs::write(&service_path, &service.code)?;
                }
            }
        }
        Ok(())
    }
}
