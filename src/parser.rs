use anyhow::{anyhow, Result};

use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;

use std::io::BufReader;
use std::str::FromStr;

use crate::servicable::*;
use crate::shape::ThingShape;
use crate::si::{self, *};
use crate::sub::Subscription;
use crate::template::ThingTemplate;
use crate::thing::Thing;
// ts: ThingShape
// sis: ServiceImplementations
// si: ServiceImplementation
// sd: ServiceDefinition
// sds: ServiceDefinitions
// ct: ConfigurationTable
// cts: ConfigurationTables
// sc: Subscription
// scs: Subscriptions
// pds: ParameterDefinitions
pub fn parse(reader: BufReader<File>, export_root: &str) -> Result<(u32, u32, u32)> {
    let mut reader = Reader::from_reader(reader);

    let mut buf = Vec::new();
    let mut thing_count = 0;
    let mut thing_template_count = 0;
    let mut thing_shape_count = 0;

    let mut found_thing = false;
    let mut found_template = false;

    // service definitions parsed from ThingShape block
    let mut in_ts = false;
    let mut in_ts_sds = false;
    let mut in_ts_sds_sd = false;
    let mut in_ts_sds_sd_pds = false;

    // service implementations parsed from Thing block
    let mut in_ts_sis = false;
    let mut in_ts_sis_si = false;
    let mut in_ts_sis_si_cts = false;
    let mut in_ts_sis_si_cts_ct = false;
    let mut in_ts_sis_si_cts_ct_rows = false;
    let mut in_ts_sis_si_cts_ct_rows_row = false;
    let mut in_ts_sis_si_cts_ct_rows_row_code = false;

    // subscriptions parsed from Thing block
    let mut in_ts_scs = false;
    let mut in_ts_scs_sc = false;
    let mut in_ts_scs_sc_si = false;
    let mut in_ts_scs_sc_si_cts = false;
    let mut in_ts_scs_sc_si_cts_ct = false;
    let mut in_ts_scs_sc_si_cts_ct_rows = false;
    let mut in_ts_scs_sc_si_cts_ct_rows_row = false;
    let mut in_ts_scs_sc_si_cts_ct_rows_row_code = false;

    let mut thing = Thing::default();
    let mut thing_shape = ThingShape::default();
    let mut thing_template = ThingTemplate::default();
    let mut service_definition = si::ServiceDefinition::default();
    let mut service_implementation = si::ServiceImplementation::default();
    let mut service_definition_map: HashMap<String, ServiceDefinition> = HashMap::new();
    let mut service_implementation_map: HashMap<String, ServiceImplementation> = HashMap::new();

    let mut subscription = Subscription::default();

    // let mut things = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if in_ts_scs_sc_si_cts_ct_rows_row && e.name() == b"code" {
                    in_ts_scs_sc_si_cts_ct_rows_row_code = true;
                }
                if in_ts_scs_sc_si_cts_ct_rows && e.name() == b"Row" {
                    in_ts_scs_sc_si_cts_ct_rows_row = true;
                }
                if in_ts_scs_sc_si_cts_ct && e.name() == b"Rows" {
                    in_ts_scs_sc_si_cts_ct_rows = true;
                }
                if in_ts_scs_sc_si_cts && e.name() == b"ConfigurationTable" {
                    in_ts_scs_sc_si_cts_ct = true;
                }
                if in_ts_scs_sc_si && e.name() == b"ConfigurationTables" {
                    in_ts_scs_sc_si_cts = true;
                }
                if in_ts_scs_sc && e.name() == b"ServiceImplementation" {
                    in_ts_scs_sc_si = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"handlerName" => {
                                subscription.service_type = ServiceHandler::from_str(
                                    &attr.unescape_and_decode_value(&reader)?,
                                )
                                .unwrap();
                            }
                            _ => {}
                        }
                    }
                }
                if in_ts_scs && e.name() == b"Subscription" {
                    in_ts_scs_sc = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"enabled" => {
                                subscription.enabled =
                                    attr.unescape_and_decode_value(&reader)? == "true";
                            }
                            b"eventName" => {
                                subscription.event_name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"name" => {
                                subscription.name = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"source" => {
                                subscription.source = attr.unescape_and_decode_value(&reader)?;
                            }
                            b"sourceProperty" => {
                                subscription.source_property =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }
                }
                if in_ts && e.name() == b"Subscriptions" {
                    in_ts_scs = true;
                }

                if in_ts_sds_sd && e.name() == b"ParameterDefinitions" {
                    in_ts_sds_sd_pds = true;
                }

                if in_ts_sds && e.name() == b"ServiceDefinition" {
                    in_ts_sds_sd = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                service_definition.name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }
                }

                if in_ts && e.name() == b"ServiceDefinitions" {
                    in_ts_sds = true;
                }
                if in_ts_sis_si_cts_ct_rows_row && e.name() == b"code" {
                    in_ts_sis_si_cts_ct_rows_row_code = true;
                }
                if in_ts_sis_si_cts_ct_rows && e.name() == b"Row" {
                    in_ts_sis_si_cts_ct_rows_row = true;
                }
                if in_ts_sis_si_cts_ct && e.name() == b"Rows" {
                    in_ts_sis_si_cts_ct_rows = true;
                }
                if in_ts_sis_si_cts && e.name() == b"ConfigurationTable" {
                    in_ts_sis_si_cts_ct = true;
                }

                if in_ts_sis_si && e.name() == b"ConfigurationTables" {
                    in_ts_sis_si_cts = true;
                }
                if in_ts_sis && e.name() == b"ServiceImplementation" {
                    in_ts_sis_si = true;
                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"name" => {
                                service_implementation.name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"handlerName" => {
                                service_implementation.service_type = ServiceHandler::from_str(
                                    &attr.unescape_and_decode_value(&reader)?,
                                )
                                .unwrap();
                            }
                            _ => {}
                        }
                    }
                    // println!("processing:{}, service:{}" , thing.name,service_implementation.name);
                }

                if in_ts && e.name() == b"ServiceImplementations" {
                    in_ts_sis = true;
                }
                if e.name() == b"ThingShape" {
                    in_ts = true;
                    if !(found_thing || found_template) {
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
                if e.name() == b"ThingTemplate" {
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
                                thing_template.template =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                if in_ts_scs_sc_si_cts_ct_rows_row && e.name() == b"code" {
                    in_ts_scs_sc_si_cts_ct_rows_row_code = false;
                }
                if in_ts_scs_sc_si_cts_ct_rows && e.name() == b"Row" {
                    in_ts_scs_sc_si_cts_ct_rows_row = false;
                }
                if in_ts_scs_sc_si_cts_ct && e.name() == b"Rows" {
                    in_ts_scs_sc_si_cts_ct_rows = false;
                }
                if in_ts_scs_sc_si_cts && e.name() == b"ConfigurationTable" {
                    in_ts_scs_sc_si_cts_ct = false;
                }
                if in_ts_scs_sc_si && e.name() == b"ConfigurationTables" {
                    in_ts_scs_sc_si_cts = false;
                }
                if in_ts_scs_sc && e.name() == b"ServiceImplementation" {
                    in_ts_scs_sc_si = false;
                }
                if in_ts_scs && e.name() == b"Subscription" {
                    in_ts_scs_sc = false;
                    if subscription.name != "" {
                        if found_thing {
                            thing.subscriptions.push(subscription);
                        } else if found_template {
                            thing_template.subscriptions.push(subscription);
                        } else {
                            thing_shape.subscriptions.push(subscription);
                        }
                    }
                    subscription = Subscription::default();
                }
                if in_ts && e.name() == b"Subscriptions" {
                    in_ts_scs = false;
                }

                if in_ts_sis_si_cts_ct_rows_row && e.name() == b"code" {
                    in_ts_sis_si_cts_ct_rows_row_code = false;
                }
                if in_ts_sis_si_cts_ct_rows && e.name() == b"Row" {
                    in_ts_sis_si_cts_ct_rows_row = false;
                }
                if in_ts_sis_si_cts_ct && e.name() == b"Rows" {
                    in_ts_sis_si_cts_ct_rows = false;
                }
                if in_ts_sis_si_cts && e.name() == b"ConfigurationTable" {
                    in_ts_sis_si_cts_ct = false;
                }

                if in_ts_sis_si && e.name() == b"ConfigurationTables" {
                    in_ts_sis_si_cts = false;
                }
                if in_ts_sis && e.name() == b"ServiceImplementation" {
                    in_ts_sis_si = false;
                    service_implementation_map
                        .insert(service_implementation.name.clone(), service_implementation);
                    service_implementation = ServiceImplementation::default();
                }
                if in_ts && e.name() == b"ServiceImplementations" {
                    in_ts_sis = false;
                }
                if in_ts_sds_sd && e.name() == b"ParameterDefinitions" {
                    in_ts_sds_sd_pds = false;
                }

                if in_ts_sds && e.name() == b"ServiceDefinition" {
                    in_ts_sds_sd = false;
                    service_definition_map
                        .insert(service_definition.name.clone(), service_definition);
                    service_definition = ServiceDefinition::default();
                }
                if in_ts && e.name() == b"ServiceDefinitions" {
                    in_ts_sds = false;
                }
                if e.name() == b"ThingShape" {
                    in_ts = false;
                    for (k, v) in service_implementation_map.drain() {
                        match service_definition_map.get(&k) {
                            Some(sd) => {
                                let mut service = Service::default();
                                service.name = k;
                                service.service_type = v.service_type;
                                service.code = v.code;
                                service.parameters = sd.parameters.clone();
                                service.result = sd.result.clone();
                                if found_thing {
                                    thing.services.push(service);
                                } else if found_template {
                                    // println!("push service:{} to template:{}",service.name,thing_template.name);
                                    thing_template.services.push(service);
                                } else {
                                    // println!("push service:{} to shape:{}",service.name,thing_shape.name);
                                    thing_shape.services.push(service);
                                }
                                // thing.services.push(service);
                            }
                            None => {}
                        }
                    }
                    if !(found_thing || found_template) {
                        thing_shape_count += 1;
                        thing_shape.export_services(export_root)?;
                        thing_shape = ThingShape::default();
                    }
                }
                if e.name() == b"Thing" {
                    found_thing = false;
                    thing.export_services(export_root)?;
                    // things.push(thing);
                    thing = Thing::default();
                }

                if e.name() == b"ThingTemplate" {
                    found_template = false;
                    thing_template.export_services(export_root)?;
                    // thing_templates.push(thing_template);
                    thing_template = ThingTemplate::default();
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::CData(ref e)) => {
                if in_ts_sis_si_cts_ct_rows_row_code {
                    let code = e.unescape_and_decode(&reader)?;
                    service_implementation.code = code;
                    // println!("{}", code);
                }
                if in_ts_scs_sc_si_cts_ct_rows_row_code {
                    let code = e.unescape_and_decode(&reader)?;
                    subscription.code = code;
                    // println!("{}", code);
                }
            }
            Ok(Event::Empty(ref e)) => {
                if in_ts_sds_sd && e.name() == b"ResultType" {
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
                    // println!("result: {:?}", service_definition.result);
                }
                if in_ts_sds_sd_pds && e.name() == b"FieldDefinition" {
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
    Ok((thing_count, thing_template_count, thing_shape_count))
}
