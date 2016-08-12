extern crate clap;
extern crate chrono;
extern crate termion;
use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};

mod generators;
use generators::*;

pub fn main() {
    let app = App::new("Rusty Rails").version(env!("CARGO_PKG_VERSION"))
        .about("Code generation tool for Rapid Application Development")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("scaffold").about("Generates a full RESTful resource")
                    .arg(Arg::with_name("resource").help("a name for the resource").required(true).takes_value(true))
                    .arg(Arg::with_name("fields").help("attributes like name:string").takes_value(true).multiple(true))
                   )
        .subcommand(SubCommand::with_name("model").about("Generates a model")
                    .arg(Arg::with_name("resource").help("a name for the model").required(true).takes_value(true))
                    .arg(Arg::with_name("fields").help("attributes like name:string").takes_value(true).multiple(true))
                   );
    let args = app.get_matches();
    match generator_for_arguments(args) {
        Ok(generator) => generator.generate(),
        Err(error) => {
            println!("{}", error.description);
            std::process::exit(1);
        }
    }
}

fn generator_for_arguments(arguments: ArgMatches) -> Result<Box<Generator+'static>, GeneratorError> {
    match arguments.subcommand_name() {
        Some("scaffold") => {
            let args = arguments.subcommand_matches("scaffold").unwrap();
            let resource = try!(Resource::from(args));
            let scaffold = try!(Scaffold::new(resource));
            Ok(Box::new(scaffold))
        },
    //    Some("model") => {
    //        let model = args.subcommand_matches("model").unwrap();
    //        let fields: Vec<Field> = Field::parse(model.values_of("fields").unwrap());
    //        let resource = Resource::new(model.value_of("resource").unwrap());
    //        generators::scaffold::model(&resource, &fields);
    //    },
    //    Some("controller") => {
    //        let model = args.subcommand_matches("controller").unwrap();
    //        let fields: Vec<Field> = Field::parse(model.values_of("fields").unwrap());
    //        let resource = Resource::new(model.value_of("resource").unwrap());
    //        generators::scaffold::model(&resource, &fields);
    //    },
        _ => unreachable!()
    }
}
