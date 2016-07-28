extern crate clap;
extern crate chrono;
extern crate ansi_term;
use clap::{Arg, App, SubCommand};

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
            let fields: Vec<Field> = Field::parse(scaffold.values_of("fields").unwrap());
            let resource = Resource::new(scaffold.value_of("resource").unwrap());
            generators::scaffold::scaffold(resource, fields);
        },
        Some("model") => {
            let model = args.subcommand_matches("model").unwrap();
            let fields: Vec<Field> = Field::parse(model.values_of("fields").unwrap());
            let resource = Resource::new(model.value_of("resource").unwrap());
            generators::scaffold::model(&resource, &fields);
        },
        Some("controller") => {
            let model = args.subcommand_matches("controller").unwrap();
            let fields: Vec<Field> = Field::parse(model.values_of("fields").unwrap());
            let resource = Resource::new(model.value_of("resource").unwrap());
            generators::scaffold::model(&resource, &fields);
        }
        _ => {}
    }

}
