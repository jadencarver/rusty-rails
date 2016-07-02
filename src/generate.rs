extern crate clap;
use clap::{Arg, App, SubCommand};
mod generators;

fn main() {
    let args = App::new("Rusty Rails").version(env!("CARGO_PKG_VERSION"))
        .about("Code generation tool for Rapid Application Development")
        .subcommand(SubCommand::with_name("scaffold").about("Generates a full RESTful resource")
                    .arg(Arg::with_name("resource").help("a name for the resource").required(true).takes_value(true))
                    .arg(Arg::with_name("fields").help("attributes like name:String").takes_value(true).multiple(true))
                   )
        .get_matches();

    match args.subcommand_name() {
        Some("scaffold") => {
            let scaffold = args.subcommand_matches("scaffold").unwrap();
            generators::scaffold::scaffold(scaffold.value_of("resource").unwrap(), scaffold.values_of("fields"))
        },
        _ => {}
    }

}
