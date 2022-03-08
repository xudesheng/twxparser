use std::path::PathBuf;

use crate::{
    si::{Service, ServiceHandler},
    sub::Subscription,
};
use anyhow::Result;
pub trait Servicable {
    fn need_export(&self) -> bool {
        self.get_valid_service_count() > 0 || self.get_valid_subscription_count() > 0
    }
    fn get_valid_service_count(&self) -> usize {
        let mut count = 0;
        for service in self.get_services() {
            if service.service_type != ServiceHandler::Reflection
                && service.service_type != ServiceHandler::Route
            {
                count += 1;
            }
        }
        count
    }
    fn get_valid_subscription_count(&self) -> usize {
        let mut count = 0;
        for subscription in self.get_subscriptions() {
            if !subscription.name.is_empty()
                && (subscription.service_type != ServiceHandler::Reflection
                    && subscription.service_type != ServiceHandler::Route)
            {
                count += 1;
            }
        }
        count
    }
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

    // entity, services, subscriptions exported.
    fn export_services(&self, root: &str) -> Result<(u32, u32, u32)> {
        if !self.need_export() {
            self.clean_folder(root)?;
            return Ok((0, 0, 0));
        }
        let mut service_count = 0;
        let mut subscription_count = 0;
        let path = self.build_entity_folder(root)?;

        if self.get_valid_subscription_count() > 0 {
            let mut path = path.clone();
            path.push("subscriptions");
            if path.exists() && path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;

            for subscription in self.get_subscriptions() {
                if subscription.service_type == ServiceHandler::Reflection
                    || subscription.service_type == ServiceHandler::Route
                    || subscription.name.is_empty()
                {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&subscription.name);
                let leading_prefix = match subscription.service_type {
                    ServiceHandler::Scrit => {
                        service_path.set_extension("js");
                        "// "
                    }
                    ServiceHandler::SQLQuery => {
                        service_path.set_extension("sql");
                        "-- "
                    }
                    ServiceHandler::SQLCommand => {
                        service_path.set_extension("sql");
                        "-- "
                    }
                    ServiceHandler::Route => {
                        service_path.set_extension("json");
                        "// "
                    }
                    ServiceHandler::Reflection => {
                        service_path.set_extension("Reflection");
                        "// "
                    }
                };
                subscription.export_to_file(&service_path, leading_prefix)?;
                subscription_count += 1;
            }
        }
        if self.get_valid_service_count() > 0 {
            let mut path = path;
            path.push("services");
            if path.exists() && path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;

            for service in self.get_services() {
                if service.service_type == ServiceHandler::Reflection
                    || service.service_type == ServiceHandler::Route
                {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&service.name);
                let leading_prefix = match service.service_type {
                    ServiceHandler::Scrit => {
                        service_path.set_extension("js");
                        "// "
                    }
                    ServiceHandler::SQLQuery => {
                        service_path.set_extension("sql");
                        "-- "
                    }
                    ServiceHandler::SQLCommand => {
                        service_path.set_extension("sql");
                        "-- "
                    }
                    ServiceHandler::Route => {
                        service_path.set_extension("json");
                        "// "
                    }
                    ServiceHandler::Reflection => {
                        service_path.set_extension("Reflection");
                        "// "
                    }
                };
                service.export_to_file(&service_path, leading_prefix)?;
                service_count += 1;
            }
        }

        Ok((1, service_count, subscription_count))
    }
}
