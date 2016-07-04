extern crate clap;
extern crate chrono;
extern crate ansi_term;
use clap::{Arg, App, SubCommand, Values};

mod generators;
use generators::*;

pub fn main() {
    let args = App::new("Rusty Rails").version(env!("CARGO_PKG_VERSION"))
        .about("Code generation tool for Rapid Application Development")
        .subcommand(SubCommand::with_name("scaffold").about("Generates a full RESTful resource")
                    .arg(Arg::with_name("resource").help("a name for the resource").required(true).takes_value(true))
                    .arg(Arg::with_name("fields").help("attributes like name:string").takes_value(true).multiple(true))
                   )
        .subcommand(SubCommand::with_name("model").about("Generates a model")
                    .arg(Arg::with_name("resource").help("a name for the model").required(true).takes_value(true))
                    .arg(Arg::with_name("fields").help("attributes like name:string").takes_value(true).multiple(true))
                   )
        .get_matches();

    match args.subcommand_name() {
        Some("scaffold") => {
            let scaffold = args.subcommand_matches("scaffold").unwrap();
            let fields: Vec<Field> = parse_fields(scaffold.values_of("fields").unwrap());
            let resource = Resource::new(scaffold.value_of("resource").unwrap());
            generators::scaffold::scaffold(resource, fields);
        },
        Some("model") => {
            let model = args.subcommand_matches("model").unwrap();
            let fields: Vec<Field> = parse_fields(model.values_of("fields").unwrap());
            let resource = Resource::new(model.value_of("resource").unwrap());
            generators::scaffold::model(&resource, &fields);
        },
        Some("controller") => {
            let model = args.subcommand_matches("controller").unwrap();
            let fields: Vec<Field> = parse_fields(model.values_of("fields").unwrap());
            let resource = Resource::new(model.value_of("resource").unwrap());
            generators::scaffold::model(&resource, &fields);
        }
        _ => {}
    }

}

fn parse_fields(attributes: Values) -> Vec<Field> {
    attributes.map( |attribute| {
        let mut split = attribute.split(':');
        let attr1 = split.next().unwrap_or("");
        let attr2 = split.next().unwrap_or("");
        let attr3 = split.next().unwrap_or("");
        if attr1 == "pub" && attr2 != "" && attr3 != "" {
            Field { // pub:name:type
                field_pub: true,
                field_name: String::from(attr2),
                field_type: String::from(attr3)
            }
        } else if attr1 == "pub" && attr2 != "" && attr3 == "" {
            Field { // pub:name
                field_pub: true,
                field_name: String::from(attr2),
                field_type: String::from(attr2)
            }
        } else if attr1 != "" && attr1 != "pub" && attr2 != "" && attr3 == "" {
            Field { // name:type
                field_pub: false,
                field_name: String::from(attr1),
                field_type: String::from(attr2)
            }
        } else if attr1 != "" && attr2 == "" && attr3 == "" {
            Field { // type
                field_pub: true,
                field_name: String::from(attr1),
                field_type: String::from(attr1)
            }
        } else { panic!("Unable to interpret field arguments!") }
    }).collect()
}

