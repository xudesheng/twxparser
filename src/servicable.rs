use std::path::PathBuf;

use crate::{
    bindings::RemotePropertyBinding,
    si::{Service, ServiceHandler},
    sub::Subscription,
};
use anyhow::Result;
use csv::WriterBuilder;
pub trait Servicable {
    fn need_export(&self) -> bool {
        self.get_valid_service_count() > 0 || self.get_valid_subscription_count() > 0
    }

    fn get_property_bindings(&self) -> &Vec<RemotePropertyBinding>;

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
        if self.get_name().is_empty() {
            return Ok(());
        }
        let mut path = PathBuf::from(root);
        path.push(self.get_charactor_str());
        path.push(&self.get_name());

        if path.exists() {
            log::trace!("cleaning folder:{}", path.display());
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

    fn export_remote_property_bindings(&self, root: &str) -> Result<()> {
        let bindings = self.get_property_bindings();
        if bindings.is_empty() {
            return Ok(());
        }

        let csvpath = crate::bindings::get_bindings_csvfile(root)?;
        let csvfile = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&csvpath)?;
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(csvfile);
        for binding in bindings.iter() {
            writer.serialize(binding)?;
        }
        writer.flush()?;
        // println!("name:{},bindings:{}", self.get_name(), bindings.len());
        Ok(())
    }
    // entity, services, subscriptions exported.
    fn export_services(&self, root: &str, should_clean: bool) -> Result<(u32, u32, u32)> {
        self.export_remote_property_bindings(root)?;

        if !self.need_export() {
            log::trace!("{} don't need export", self.get_name());
            self.clean_folder(root)?;
            return Ok((0, 0, 0));
        }
        log::trace!("{} would need export", self.get_name());
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
            log::trace!(
                "{} export subscriptions to:{}",
                self.get_name(),
                path.display()
            );
            for subscription in self.get_subscriptions() {
                if subscription.service_type == ServiceHandler::Reflection
                    || subscription.service_type == ServiceHandler::Route
                    || subscription.name.is_empty()
                {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&subscription.name.replace(':', "_"));
                let leading_prefix = match subscription.service_type {
                    ServiceHandler::Script => {
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
                match subscription.export_to_file(&service_path, leading_prefix, should_clean) {
                    Ok(_) => {
                        subscription_count += 1;
                    }
                    Err(e) => {
                        log::error!(
                            "export {} to path:{} failed:{}",
                            subscription.name,
                            service_path.display(),
                            e
                        );
                    }
                }
                // subscription_count += 1;
            }
        }
        if self.get_valid_service_count() > 0 {
            let mut path = path;
            path.push("services");
            if path.exists() && path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
            std::fs::create_dir_all(&path)?;
            log::trace!("{} export services to:{}", self.get_name(), path.display());
            for service in self.get_services() {
                if service.service_type == ServiceHandler::Reflection
                    || service.service_type == ServiceHandler::Route
                {
                    continue;
                }
                let mut service_path = path.clone();
                service_path.push(&service.name.replace(':', "_"));
                let leading_prefix = match service.service_type {
                    ServiceHandler::Script => {
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
                match service.export_to_file(&service_path, leading_prefix, should_clean) {
                    Ok(_) => {
                        service_count += 1;
                    }
                    Err(e) => {
                        log::error!(
                            "export service {} to path:{} failed:{}",
                            service.name,
                            service_path.display(),
                            e
                        );
                    }
                }
                // service_count += 1;
            }
        }

        Ok((1, service_count, subscription_count))
    }
}
