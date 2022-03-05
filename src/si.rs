use anyhow::Result;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceHandler {
    Scrit,
    SQLQuery,
    SQLCommand,
    Route,
    Reflection,
}

impl Default for ServiceHandler {
    fn default() -> Self {
        ServiceHandler::Scrit
    }
}
impl fmt::Display for ServiceHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceHandler::Scrit => write!(f, "Scrit"),
            ServiceHandler::SQLQuery => write!(f, "SQLQuery"),
            ServiceHandler::SQLCommand => write!(f, "SQLCommand"),
            ServiceHandler::Route => write!(f, "Route"),
            ServiceHandler::Reflection => write!(f, "Reflection"),
        }
    }
}

impl FromStr for ServiceHandler {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Script" => Ok(ServiceHandler::Scrit),
            "SQLQuery" => Ok(ServiceHandler::SQLQuery),
            "SQLCommand" => Ok(ServiceHandler::SQLCommand),
            "Route" => Ok(ServiceHandler::Route),
            "Reflection" => Ok(ServiceHandler::Reflection),
            _ => panic!("unknown service handler:{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FieldDefinition {
    pub name: String,
    pub base_type: String,
    pub ordinal: u32,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ServiceImplementation {
    pub name: String,
    pub service_type: ServiceHandler,
    pub code: String,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ServiceDefinition {
    pub name: String,
    pub parameters: Vec<FieldDefinition>,
    pub result: Option<FieldDefinition>,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Service {
    pub name: String,
    pub service_type: ServiceHandler,
    pub code: String,
    pub parameters: Vec<FieldDefinition>,
    pub result: Option<FieldDefinition>,
}
