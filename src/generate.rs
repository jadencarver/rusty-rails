extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let args = App::new("Rusty Rails").version(env!("CARGO_PKG_VERSION"))
        .about("Code generation tool for Rapid Application Development")
        .subcommand(SubCommand::with_name("scaffold").about("Generates a full RESTful resource")
                    .arg(Arg::with_name("resource").help("a name for the resource").required(true))
                    .arg(Arg::with_name("fields").help("attributes like name:String"))
                   )
        .get_matches();

    match args.subcommand_name() {
        Some("scaffold") => println!("Scaffold generator started"),
        _ => {}
    }

}
