use crate::servicable::Servicable;
use crate::si::*;
use crate::sub::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thing {
    pub name: String,
    pub template: String,
    pub services: Vec<Service>,
    pub subscriptions: Vec<Subscription>,
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

    fn get_charactor_str(&self) -> &'static str {
        "Things"
    }
}
