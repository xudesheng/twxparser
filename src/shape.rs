use crate::si::*;
use anyhow::Result;
use std::path::PathBuf;

pub const KEY_NAME: &str = "thingShapes";
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThingShape {
    pub name: String,
    pub template: String,
    pub services: Vec<Service>,
}

impl ThingShape {
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

    pub fn get_real_service_count(&self) -> usize {
        let mut count = 0;
        for service in &self.services {
            if service.service_type != ServiceHandler::Reflection {
                count += 1;
            }
        }
        count
    }

    pub fn export_services(&self, root: &str) -> Result<()> {
        self.clean_folder(root)?;
        if self.services.len() == 0 || self.get_real_service_count() == 0 {
            return Ok(());
        }
        self.build_folder(root)?;

        let mut path = PathBuf::from(root);
        path.push(KEY_NAME);
        path.push(&self.name);

        for service in &self.services {
            if service.name == "" {
                continue;
            }
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
                    continue;
                }
            }
        }
        Ok(())
    }
}
