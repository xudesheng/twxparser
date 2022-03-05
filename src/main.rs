mod si;
mod thing;
mod template;
mod shape;

use anyhow::{anyhow, Result};

use quick_xml::events::Event;
use quick_xml::Reader;
use std::str::FromStr;
use std::{fs, io::BufReader};
use std::collections::HashMap;

use crate::si::*;
use crate::thing::Thing;
use crate::shape::ThingShape;
use crate::template::ThingTemplate;

pub const TEST_FILE: &str = "./sample/AllEntities.xml";
pub const ROOT_EXPORT:&str = "./sample/";
fn main() -> Result<()> {
    // println!("Hello, world!");
    // env_logger::init();
    println!("reading file:{}", TEST_FILE);
    let file = fs::File::open(TEST_FILE)?;
    let file = BufReader::new(file);
    let mut reader = Reader::from_reader(file);

    let mut buf = Vec::new();
    let mut thing_count = 0;
    let mut thing_template_count = 0;
    let mut thing_shape_count = 0;

    let mut found_thing = false;
    let mut found_template = false;


    let mut in_thingshape = false;
    let mut in_thingshape_servicedefinitions = false;
    let mut in_thingshape_servicedefinitions_servicedefinition = false;
    let mut in_thingshape_servicedefinitions_servicedefinition_parameterdefinitions =
        false;

    let mut in_thingshape_serviceimplementations = false;
    let mut in_thingshape_serviceimplementations_serviceimplementation = false;
    let mut
    in_thingshape_serviceimplementations_serviceimplementation_configurationtables =
        false;
    let mut
    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable =
        false;
    let mut
    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows =
        false;
    let mut
    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row =
        false;
    let mut in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row_code = false;
    
    
    let mut thing = Thing::default();
    let mut thing_shape = ThingShape::default();
    let mut thing_template = ThingTemplate::default();
    let mut service_definition = si::ServiceDefinition::default();
    let mut service_implementation = si::ServiceImplementation::default();
    let mut service_definition_map:HashMap<String,ServiceDefinition> = HashMap::new();
    let mut service_implementation_map:HashMap<String,ServiceImplementation> = HashMap::new();

    // let mut things = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if in_thingshape_servicedefinitions_servicedefinition_parameterdefinitions
                    && e.name() == b"FieldDefinition"
                {
                    let mut field_definition = si::FieldDefinition::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                field_definition.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"baseType" => {
                                field_definition.base_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"ordinal" => {
                                field_definition.ordinal =
                                    attr.unescape_and_decode_value(&reader)?.parse::<u32>()?;
                            }
                            _ => {}
                        }
                    }
                    service_definition.parameters.push(field_definition);
                }
                if in_thingshape_servicedefinitions_servicedefinition
                    && e.name() == b"ParameterDefinitions"
                {
                    in_thingshape_servicedefinitions_servicedefinition_parameterdefinitions = true;
                }
                if in_thingshape_servicedefinitions_servicedefinition
                    && e.name() == b"ResultType"
                {
                    let mut field_definition = si::FieldDefinition::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                field_definition.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"baseType" => {
                                field_definition.base_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"ordinal" => {
                                field_definition.ordinal =
                                    attr.unescape_and_decode_value(&reader)?.parse::<u32>()?;
                            }
                            _ => {}
                        }
                    }
                    service_definition.result = Some(field_definition);
                }
                if in_thingshape_servicedefinitions && e.name() == b"ServiceDefinition" {
                    in_thingshape_servicedefinitions_servicedefinition = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                service_definition.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            _ =>{}
                        }
                    }
                }

                if in_thingshape && e.name() == b"ServiceDefinitions" {
                    in_thingshape_servicedefinitions = true;
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row && e.name() == b"code" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row_code = true;
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows && e.name()==b"Row" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row = true;

                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable && e.name() == b"Rows" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows = true;
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables && e.name() == b"ConfigurationTable" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable = true;
                }

                if in_thingshape_serviceimplementations_serviceimplementation
                    && e.name() == b"ConfigurationTables"
                {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables = true;
                }
                if in_thingshape_serviceimplementations
                    && e.name() == b"ServiceImplementation"
                {
                    in_thingshape_serviceimplementations_serviceimplementation = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                service_implementation.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"handlerName"=>{
                                service_implementation.service_type = ServiceHandler::from_str(&attr.unescape_and_decode_value(&reader)?).unwrap();
                            }
                            _ =>{}
                        }
                    }
                    // println!("processing:{}, service:{}" , thing.name,service_implementation.name);
                }

                if in_thingshape && e.name() == b"ServiceImplementations" {
                    in_thingshape_serviceimplementations = true;
                    
                }
                if e.name() == b"ThingShape" {
                    in_thingshape = true;
                    if !(found_thing || found_template){
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key {
                                b"name" => {
                                    thing_shape.name = attr.unescape_and_decode_value(&reader)?;
                                }
                                
                                _ => {}
                            }
                        }
                        // println!("processing shape:{}", thing_shape.name);
                    }
                }
                if e.name() == b"Thing" {
                    thing_count += 1;
                    found_thing = true;
                    // thing = Thing::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                thing.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"thingTemplate" => {
                                thing.template = attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }
                    // thing.build_folder(ROOT_EXPORT)?;
                }
                if e.name() == b"ThingTemplate"{
                    thing_template_count += 1;
                    found_template = true;
                    // thing = Thing::default();
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                thing_template.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"thingTemplate" => {
                                thing_template.template = attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row && e.name() == b"code" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row_code = false;
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows && e.name()==b"Row" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row = false;
                    
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable && e.name() == b"Rows" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows = false;
                }
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables && e.name() == b"ConfigurationTable" {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable = false;
                }

                if in_thingshape_serviceimplementations_serviceimplementation
                    && e.name() == b"ConfigurationTables"
                {
                    in_thingshape_serviceimplementations_serviceimplementation_configurationtables = false;
                }
                if in_thingshape_serviceimplementations
                    && e.name() == b"ServiceImplementation"
                {
                    in_thingshape_serviceimplementations_serviceimplementation = false;
                    service_implementation_map.insert(service_implementation.name.clone(), service_implementation);
                    service_implementation = ServiceImplementation::default();
                }
                if in_thingshape && e.name() == b"ServiceImplementations" {
                    in_thingshape_serviceimplementations = false;
                }
                if in_thingshape_servicedefinitions_servicedefinition
                    && e.name() == b"ParameterDefinitions"
                {
                    in_thingshape_servicedefinitions_servicedefinition_parameterdefinitions = false;
                }
                if in_thingshape_servicedefinitions && e.name() == b"ServiceDefinition" {
                    in_thingshape_servicedefinitions_servicedefinition = false;
                    service_definition_map.insert(service_definition.name.clone(), service_definition);
                    service_definition = ServiceDefinition::default();
                }
                if in_thingshape && e.name() == b"ServiceDefinitions" {
                    in_thingshape_servicedefinitions = false;
                }
                if e.name() == b"ThingShape" {
                    in_thingshape = false;
                    for (k,v) in service_implementation_map.drain() {
                        match service_definition_map.get(&k) {
                            Some(sd) => {
                                let mut service = Service::default();
                                service.name = k;
                                service.service_type = v.service_type;
                                service.code = v.code;
                                service.parameters = sd.parameters.clone();
                                service.result = sd.result.clone();
                                if found_thing{
                                    thing.services.push(service);
                                }else  if found_template{
                                    // println!("push service:{} to template:{}",service.name,thing_template.name);
                                    thing_template.services.push(service);
                                }else{
                                    // println!("push service:{} to shape:{}",service.name,thing_shape.name);
                                    thing_shape.services.push(service);
                                }
                                // thing.services.push(service);

                            },
                            None => {},
                        }
                    }
                    if !(found_thing || found_template) {
                        thing_shape_count += 1;
                        thing_shape.export_services(ROOT_EXPORT)?;
                        thing_shape = ThingShape::default();
                    }
                }
                if e.name() == b"Thing" {
                    found_thing = false;
                    thing.export_services(ROOT_EXPORT)?;
                    // things.push(thing);
                    thing = Thing::default();
                }

                if e.name() == b"ThingTemplate" {
                    found_template = false;
                    thing_template.export_services(ROOT_EXPORT)?;
                    // thing_templates.push(thing_template);
                    thing_template = ThingTemplate::default();
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::CData(ref e))=>{
                if in_thingshape_serviceimplementations_serviceimplementation_configurationtables_configurationtable_rows_row_code{
                    let code = e.unescape_and_decode(&reader)?;
                    service_implementation.code = code;
                    // println!("{}", code);
                }
            }
            Err(e) => {
                return Err(anyhow!(
                    "Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                ));
                // break;
            }
            _ => (),
        }
    }
    println!("total things:{}, templates:{}, shapes:{}", thing_count, thing_template_count, thing_shape_count);
    
    Ok(())
}
