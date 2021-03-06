use crate::bindings::RemotePropertyBinding;
use crate::servicable::Servicable;
use crate::si::*;
use crate::sub::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThingShape {
    pub name: String,
    pub template: String,
    pub services: Vec<Service>,
    pub subscriptions: Vec<Subscription>,
    pub property_bindings: Vec<RemotePropertyBinding>,
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

    fn get_charactor_str(&self) -> &'static str {
        "ThingShapes"
    }

    fn get_property_bindings(&self) -> &Vec<RemotePropertyBinding> {
        &self.property_bindings
    }
}
