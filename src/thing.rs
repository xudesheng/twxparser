use crate::si::*;
use crate::sub::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thing {
    pub name: String,
    pub template: String,
    pub services: Vec<Service>,
    pub subscriptions: Vec<Subscription>,
}
