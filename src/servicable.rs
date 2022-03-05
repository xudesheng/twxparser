use std::path::PathBuf;

use crate::{
    shape::ThingShape,
    si::{Service, ServiceHandler},
    sub::Subscription,
    template::ThingTemplate,
    thing::Thing,
};
use anyhow::Result;
pub trait Servicable {
    fn need_export(&self) -> bool {
        self.get_valid_service_count() > 0 || self.get_valid_subscription_count() > 0
    }
    fn get_valid_service_count(&self) -> usize;
    fn get_valid_subscription_count(&self) -> usize;
    fn get_charactor_str(&self) -> &'static str;
    fn get_name(&self) -> &str;
    fn get_services(&self) -> &Vec<Service>;
    fn get_subscriptions(&self) -> &Vec<Subscription>;

    fn clean_folder(&self, root: &str) -> Result<()> {
        let mut path = PathBuf::from(root);
        path.push(self.get_charactor_str());
        path.push(&self.get_name());

        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        Ok(())
    }

    fn build_entity_folder(&self, root: &str) -> Result<PathBuf> {
        let mut path = PathBuf::from(root);
        path.push(self.get_charactor_str());
        path.push(&self.get_name());

        if path.exists() && path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;

        Ok(path)
    }

    fn export_services(&self, root: &str) -> Result<()> {
        if !self.need_export() {
            self.clean_folder(root)?;
            return Ok(());
        }
        let path = self.build_entity_folder(root)?;

        if self.get_valid_subscription_count() > 0 {
            let mut path = path.clone();
            path.push("subscriptions");
            if path.exists() && path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;

            for subscription in self.get_subscriptions() {
                if subscription.service_type == ServiceHandler::Reflection {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&subscription.name);
                match subscription.service_type {
                    ServiceHandler::Scrit => {
                        service_path.set_extension("js");
                    }
                    ServiceHandler::SQLQuery => {
                        service_path.set_extension("sql");
                    }
                    ServiceHandler::SQLCommand => {
                        service_path.set_extension("sql");
                    }
                    ServiceHandler::Route => {
                        service_path.set_extension("json");
                    }
                    ServiceHandler::Reflection => {
                        service_path.set_extension("Reflection");
                    }
                }
                subscription.export_to_file(&service_path)?;
            }
        }
        if self.get_valid_service_count() > 0 {
            let mut path = path.clone();
            path.push("services");
            if path.exists() && path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;

            for service in self.get_services() {
                if service.service_type == ServiceHandler::Reflection {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&service.name);
                match service.service_type {
                    ServiceHandler::Scrit => {
                        service_path.set_extension("js");
                    }
                    ServiceHandler::SQLQuery => {
                        service_path.set_extension("sql");
                    }
                    ServiceHandler::SQLCommand => {
                        service_path.set_extension("sql");
                    }
                    ServiceHandler::Route => {
                        service_path.set_extension("json");
                    }
                    ServiceHandler::Reflection => {
                        service_path.set_extension("Reflection");
                    }
                }
                service.export_to_file(&service_path)?;
            }
        }

        Ok(())
    }
}

impl Servicable for Thing {
    fn get_services(&self) -> &Vec<Service> {
        &self.services
    }
    fn get_subscriptions(&self) -> &Vec<Subscription> {
        &self.subscriptions
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_valid_service_count(&self) -> usize {
        let mut count = 0;
        for service in &self.services {
            if service.service_type != ServiceHandler::Reflection {
                count += 1;
            }
        }
        count
    }
    fn get_valid_subscription_count(&self) -> usize {
        let mut count = 0;
        for subscription in &self.subscriptions {
            if subscription.name != "" {
                count += 1;
            }
        }
        count
    }
    fn get_charactor_str(&self) -> &'static str {
        "Things"
    }
}

impl Servicable for ThingShape {
    fn get_services(&self) -> &Vec<Service> {
        &self.services
    }
    fn get_subscriptions(&self) -> &Vec<Subscription> {
        &self.subscriptions
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_valid_service_count(&self) -> usize {
        let mut count = 0;
        for service in &self.services {
            if service.service_type != ServiceHandler::Reflection {
                count += 1;
            }
        }
        count
    }
    fn get_valid_subscription_count(&self) -> usize {
        let mut count = 0;
        for subscription in &self.subscriptions {
            if subscription.name != "" {
                count += 1;
            }
        }
        count
    }
    fn get_charactor_str(&self) -> &'static str {
        "ThingShapes"
    }
}

impl Servicable for ThingTemplate {
    fn get_services(&self) -> &Vec<Service> {
        &self.services
    }
    fn get_subscriptions(&self) -> &Vec<Subscription> {
        &self.subscriptions
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_valid_service_count(&self) -> usize {
        let mut count = 0;
        for service in &self.services {
            if service.service_type != ServiceHandler::Reflection {
                count += 1;
            }
        }
        count
    }
    fn get_valid_subscription_count(&self) -> usize {
        let mut count = 0;
        for subscription in &self.subscriptions {
            if subscription.name != "" {
                count += 1;
            }
        }
        count
    }
    fn get_charactor_str(&self) -> &'static str {
        "ThingTemplates"
    }
}
