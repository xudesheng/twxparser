use anyhow::Result;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use std::str::FromStr;
use std::{fmt, path::Path};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ServiceHandler {
    Script,
    SQLQuery,
    SQLCommand,
    Route,
    Reflection,
}

impl Default for ServiceHandler {
    fn default() -> Self {
        ServiceHandler::Script
    }
}
impl fmt::Display for ServiceHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceHandler::Script => write!(f, "Script"),
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
            "Script" => Ok(ServiceHandler::Script),
            "SQLQuery" => Ok(ServiceHandler::SQLQuery),
            "SQLCommand" => Ok(ServiceHandler::SQLCommand),
            "Route" => Ok(ServiceHandler::Route),
            "Reflection" => Ok(ServiceHandler::Reflection),
            _ => panic!("unknown service handler:{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FieldDefinition {
    pub name: String,
    pub base_type: String,
    pub ordinal: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServiceImplementation {
    pub name: String,
    pub service_type: ServiceHandler,
    pub code: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServiceDefinition {
    pub name: String,
    pub parameters: Vec<FieldDefinition>,
    pub result: Option<FieldDefinition>,
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Service {
    pub name: String,
    pub service_type: ServiceHandler,
    pub code: String,
    pub parameters: Vec<FieldDefinition>,
    pub result: Option<FieldDefinition>,
}

impl Service {
    pub fn export_to_file(&self, path: &Path, leading_prefix: &str) -> Result<()> {
        log::trace!(
            "exporting service to file:{}, leading prefix:{}",
            path.display(),
            leading_prefix
        );
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}{:>20}:\t{}", leading_prefix, "name", self.name)?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix, "service_type", self.service_type
        )?;
        writeln!(
            writer,
            "{}{:>20}:\t{}",
            leading_prefix,
            "parameters",
            self.parameters.len()
        )?;
        for parameter in self.parameters.iter() {
            writeln!(
                writer,
                "{}\t\t\t\t{:<20}:\t{}",
                leading_prefix, parameter.name, parameter.base_type
            )?;
        }

        // writeln!(writer, "{}", &self.code)?;
        writeln!(writer, "{}", clean_prettified_code(&self.code))?;
        if let Some(ref result) = self.result {
            write!(
                writer,
                "{}{:>20}:\t{}",
                leading_prefix, result.name, result.base_type
            )?;
        }

        Ok(())
    }
}

fn clean_prettified_code(code: &str) -> String {
    let should_clean = match std::env::var("CLEAN_PRETTIFIED_CODE") {
        Ok(content) => content.to_lowercase() != "false",
        _ => true,
    };
    if !should_clean {
        return code.to_string();
    }
    let leading_counter = probe_leading_letter_count(code);
    if leading_counter == 0 {
        return code.to_string();
    }
    let mut result = String::new();
    let lines = code.split('\n').collect::<Vec<&str>>();
    if lines.len() < 2 {
        return code.to_string();
    }
    result.push_str(lines[0]);
    for line in lines.iter().skip(1) {
        result.push('\n');
        if line.len() < leading_counter {
            result.push_str(line);
        } else {
            result.push_str(&line[leading_counter..]);
        }
        // result.push_str("\r");
    }
    result
}
fn probe_leading_letter_count(code: &str) -> usize {
    let lines = code.split('\n').collect::<Vec<&str>>();
    // println!("len:{},lines:{:?}", lines.len(), lines);
    if lines.is_empty() || lines.len() < 2 {
        return 0;
    }
    let mut count = 0;
    for line in &lines[1..] {
        if line.is_empty() {
            continue;
        }
        let mut line_white_count = 0;
        for char in line.chars() {
            if char.is_whitespace() {
                line_white_count += 1;
            } else {
                break;
            }
        }
        if count == 0 || line_white_count < count {
            count = line_white_count;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use quick_xml::{events::Event, Reader};

    use super::*;

    #[test]
    fn test_clean_prettified_code() {
        let code = r#"
        token = token || ""; // Default to "unset"
        
        var encrypted = Resources["EncryptionServices"].EncryptPropertyValue({
            data: token
        });
        me.token = encrypted;
        "#;
        let counter = probe_leading_letter_count(code);
        assert_eq!(counter, 8);
    }

    #[test]
    fn test_clean_code() {
        let code = r#"
        token = token || ""; // Default to "unset"
        
        var encrypted = Resources["EncryptionServices"].EncryptPropertyValue({
            data: token
        });
        me.token = encrypted;
        "#;
        let cleaned_code = r#"
token = token || ""; // Default to "unset"

var encrypted = Resources["EncryptionServices"].EncryptPropertyValue({
    data: token
});
me.token = encrypted;
"#;
        let result = clean_prettified_code(code);
        assert_eq!(result, cleaned_code);
    }
    #[test]
    fn test_escaped_xml() {
        let source = r#"<?xml version="1.0" encoding="UTF-8"?>
        <Entities
         majorVersion="9"
         minorVersion="3"
         universal="">
         <code>
                                        <![CDATA[
                                        // New token is no longer invalid
                                        me.tokenInvalid = false;
                                        
                                        // Request connection servers to remove old token from cache
                                        var target = source;
                                        logger.debug("Token value changed; clearing SecurityClaimsStash for key '" + target + "'");
                                        
                                        Things["ConnectionServicesHub"].ClearCacheEntry({
                                            cacheName: "SecurityClaimsStash",
                                            cacheKey: target
                                        });
                                        ]]>
                                        </code>
         </Entities>"#;
        let mut reader = Reader::from_str(source);
        let mut buf = Vec::new();
        let mut found = false;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name() == b"code" {
                        println!("start content:{:?}", e);
                        found = true;
                    }
                }
                Ok(Event::CData(ref e)) => {
                    let content = e.unescaped().unwrap();
                    let content = String::from_utf8(content.to_vec()).unwrap();
                    println!("content:{:?}", content);
                    found = true;
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        assert!(found);
    }
}
