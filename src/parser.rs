use anyhow::{anyhow, Result};

use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;

use std::io::BufReader;
use std::str::FromStr;

use crate::shape::ThingShape;
use crate::si::{self, *};
use crate::sub::Subscription;
use crate::template::ThingTemplate;
use crate::thing::Thing;
use crate::{bindings::RemotePropertyBinding, servicable::*};
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
type ParserCounters = (
    u32, // total things
    u32, // total thing templates
    u32, // total thing shapes
    u32, // exported things
    u32, // exported thing templates
    u32, // exported thing shapes
    u32, // exported services
    u32, // exported subscriptions
);
pub fn parse(reader: BufReader<File>, export_root: &str) -> Result<ParserCounters> {
    let mut reader = Reader::from_reader(reader);

    let mut buf = Vec::new();
    let mut thing_count = 0;
    let mut thing_template_count = 0;
    let mut thing_shape_count = 0;

    let mut exported_things = 0;
    let mut exported_thing_templates = 0;
    let mut exported_thing_shapes = 0;
    let mut exported_services = 0;
    let mut exported_subscriptions = 0;

    let mut found_thing = false;
    let mut found_template = false;

    let mut is_remote_property_bindings = false; // remote property bindings.

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
    // for sql query and sql command
    let mut in_ts_sis_si_cts_ct_rows_row_sql = false;

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
                        if let b"handlerName" = attr.key {
                            subscription.service_type =
                                ServiceHandler::from_str(&attr.unescape_and_decode_value(&reader)?)
                                    .unwrap();
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
                        if let b"name" = attr.key {
                            service_definition.name = attr.unescape_and_decode_value(&reader)?;
                        }
                    }
                }

                if in_ts && e.name() == b"ServiceDefinitions" {
                    in_ts_sds = true;
                }
                if in_ts_sis_si_cts_ct_rows_row && e.name() == b"sql" {
                    in_ts_sis_si_cts_ct_rows_row_sql = true;
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
                    log::debug!(
                        "processing:{}, service:{}",
                        thing.name,
                        service_implementation.name
                    );
                }

                if in_ts && e.name() == b"ServiceImplementations" {
                    in_ts_sis = true;
                }
                if e.name() == b"ThingShape" {
                    in_ts = true;
                    if !(found_thing || found_template) {
                        for attr in e.attributes() {
                            let attr = attr?;
                            if let b"name" = attr.key {
                                thing_shape.name = attr.unescape_and_decode_value(&reader)?;
                            }
                        }
                        // println!("processing shape:{}", thing_shape.name);
                    }
                }

                if is_remote_property_bindings && e.name() == b"RemotePropertyBinding" {
                    // println!("found remote property binding");
                    let mut remote_property_binding = RemotePropertyBinding::default();

                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"aspect.dataShape" => {
                                remote_property_binding.data_shape =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.industrialDataType" => {
                                remote_property_binding.industrial_data_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.scanRate" => {
                                remote_property_binding.scan_rate =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.startType" => {
                                remote_property_binding.start_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.tagAddress" => {
                                remote_property_binding.tag_address =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.tagType" => {
                                remote_property_binding.tag_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"foldType" => {
                                remote_property_binding.fold_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"name" => {
                                remote_property_binding.property_name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"pushType" => {
                                remote_property_binding.push_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"sourceName" => {
                                remote_property_binding.source_name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"timeout" => {
                                remote_property_binding.timeout =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }

                    if found_thing {
                        remote_property_binding.entity_type = "Thing".to_string();
                        remote_property_binding.entity_name = thing.name.clone();

                        thing.property_bindings.push(remote_property_binding);
                    } else if found_template {
                        remote_property_binding.entity_type = "ThingTemplate".to_string();
                        remote_property_binding.entity_name = thing_template.name.clone();
                        thing_template
                            .property_bindings
                            .push(remote_property_binding);
                    } else if in_ts {
                        remote_property_binding.entity_type = "ThingShape".to_string();
                        remote_property_binding.entity_name = thing_shape.name.clone();
                        thing_shape.property_bindings.push(remote_property_binding);
                    }
                }
                if e.name() == b"RemotePropertyBindings" {
                    // println!("found remote property bindings");
                    is_remote_property_bindings = true;
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
                    if !subscription.name.is_empty() {
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

                if in_ts_sis_si_cts_ct_rows_row && e.name() == b"sql" {
                    in_ts_sis_si_cts_ct_rows_row_sql = false;
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
                        if let Some(sd) = service_definition_map.get(&k) {
                            let service = Service {
                                name: k,
                                service_type: v.service_type,
                                code: v.code,
                                parameters: sd.parameters.clone(),
                                result: sd.result.clone(),
                            };

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
                    }
                    if !(found_thing || found_template) {
                        log::trace!("found shape:{}, exporting", thing_shape.name);
                        thing_shape_count += 1;
                        let (entity_count, svc_count, sub_count) =
                            match thing_shape.export_services(export_root) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!(
                                        "export_services error:{},thing_shape.name:{}",
                                        e, thing_shape.name
                                    );
                                    continue;
                                }
                            };
                        exported_thing_shapes += entity_count;
                        exported_services += svc_count;
                        exported_subscriptions += sub_count;
                        thing_shape = ThingShape::default();
                    }
                }
                if e.name() == b"RemotePropertyBindings" {
                    is_remote_property_bindings = false;
                }
                if e.name() == b"Thing" {
                    found_thing = false;
                    let (entity_count, svc_count, sub_count) =
                        match thing.export_services(export_root) {
                            Ok(v) => v,
                            Err(e) => {
                                println!("export_services error:{},thing.name:{}", e, thing.name);
                                continue;
                            }
                        };
                    exported_things += entity_count;
                    exported_services += svc_count;
                    exported_subscriptions += sub_count;
                    // things.push(thing);
                    thing = Thing::default();
                }

                if e.name() == b"ThingTemplate" {
                    found_template = false;
                    let (entity_count, svc_count, sub_count) =
                        match thing_template.export_services(export_root) {
                            Ok(v) => v,
                            Err(e) => {
                                println!(
                                    "export_services error:{},thing_template.name:{}",
                                    e, thing_template.name
                                );
                                continue;
                            }
                        };
                    exported_thing_templates += entity_count;
                    exported_services += svc_count;
                    exported_subscriptions += sub_count;
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
                // this block can be merged with the above block. Keep it for logic simplicity.
                if in_ts_sis_si_cts_ct_rows_row_sql {
                    let sql = e.unescape_and_decode(&reader)?;
                    // println!("{}", sql);
                    service_implementation.code = sql;
                }
                if in_ts_scs_sc_si_cts_ct_rows_row_code {
                    let code = e.unescape_and_decode(&reader)?;
                    subscription.code = code;
                    // println!("{}", code);
                }
            }
            Ok(Event::Empty(ref e)) => {
                if is_remote_property_bindings && e.name() == b"RemotePropertyBinding" {
                    let mut remote_property_binding = RemotePropertyBinding::default();

                    for attr in e.attributes() {
                        let attr = attr?;
                        match attr.key {
                            b"aspect.dataShape" => {
                                remote_property_binding.data_shape =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.industrialDataType" => {
                                remote_property_binding.industrial_data_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.scanRate" => {
                                remote_property_binding.scan_rate =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.startType" => {
                                remote_property_binding.start_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.tagAddress" => {
                                remote_property_binding.tag_address =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"aspect.tagType" => {
                                remote_property_binding.tag_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"foldType" => {
                                remote_property_binding.fold_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"name" => {
                                remote_property_binding.property_name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"pushType" => {
                                remote_property_binding.push_type =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"sourceName" => {
                                remote_property_binding.source_name =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            b"timeout" => {
                                remote_property_binding.timeout =
                                    attr.unescape_and_decode_value(&reader)?;
                            }
                            _ => {}
                        }
                    }

                    if found_thing {
                        remote_property_binding.entity_type = "Thing".to_string();
                        remote_property_binding.entity_name = thing.name.clone();

                        thing.property_bindings.push(remote_property_binding);
                    } else if found_template {
                        remote_property_binding.entity_type = "ThingTemplate".to_string();
                        remote_property_binding.entity_name = thing_template.name.clone();
                        thing_template
                            .property_bindings
                            .push(remote_property_binding);
                    } else if in_ts {
                        remote_property_binding.entity_type = "ThingShape".to_string();
                        remote_property_binding.entity_name = thing_shape.name.clone();
                        thing_shape.property_bindings.push(remote_property_binding);
                    }
                }
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
    Ok((
        thing_count,
        thing_template_count,
        thing_shape_count,
        exported_things,
        exported_thing_templates,
        exported_thing_shapes,
        exported_services,
        exported_subscriptions,
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remote_property_bindings() {
        use quick_xml::events::Event;
        use quick_xml::Reader;
        let xml = r#"
        <RemotePropertyBindings>
        <RemotePropertyBinding aspect.dataShape="" aspect.industrialDataType="Short" aspect.scanRate="1000" aspect.startType="readEdgeValue" aspect.tagAddress="HNK.Bay6.ControllerMode" aspect.tagType="Static" foldType="NONE" name="ControllerMode" pushType="VALUE" sourceName="" timeout="0"/>
        <RemotePropertyBinding aspect.dataShape="" aspect.industrialDataType="Byte" aspect.scanRate="1000" aspect.startType="readEdgeValue" aspect.tagAddress="HNK.Bay6.Feedrate Override" aspect.tagType="Static" foldType="NONE" name="FeedrateOverride" pushType="VALUE" sourceName="" timeout="0"/>
        <RemotePropertyBinding aspect.dataShape="" aspect.industrialDataType="Short" aspect.scanRate="1000" aspect.startType="readEdgeValue" aspect.tagAddress="HNK.Bay6.MachineStatus" aspect.tagType="Static" foldType="NONE" name="LocalMachineSts" pushType="VALUE" sourceName="" timeout="0"/>
        <RemotePropertyBinding aspect.dataShape="" aspect.industrialDataType="Float" aspect.scanRate="1000" aspect.startType="readEdgeValue" aspect.tagAddress="HNK.Bay6.PartCount" aspect.tagType="Static" foldType="NONE" name="LocalPartCount" pushType="VALUE" sourceName="" timeout="0"/>
        <RemotePropertyBinding aspect.dataShape="" aspect.industrialDataType="Float" aspect.scanRate="1000" aspect.startType="readEdgeValue" aspect.tagAddress="HNK.Bay6.ProgramName" aspect.tagType="Static" foldType="NONE" name="ProgramName" pushType="VALUE" sourceName="" timeout="0"/>
      </RemotePropertyBindings>
        "#;
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut count = 0;
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    println!("{:?}", e);
                    println!("name: {}", std::str::from_utf8(e.name()).unwrap());

                    if e.name() == b"RemotePropertyBinding" {
                        count += 1;
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    println!("{:?}", e);
                    println!("name: {}", std::str::from_utf8(e.name()).unwrap());
                    if e.name() == b"RemotePropertyBinding" {
                        count += 1;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }

        assert_eq!(count, 5);
    }
}
