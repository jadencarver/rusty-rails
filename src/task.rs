extern crate clap;
extern crate glob;

use clap::{Arg, App, SubCommand};
use glob::glob;
use std::fs;

mod tasks;

fn main() {
    let args = App::new("Rusty Rails").version(env!("CARGO_PKG_VERSION"))
        .about("Runs tasks related to managing your Rusty Rails environment")
        .arg(Arg::with_name("verbose").short("v").help("Verbose output"))

        .subcommand(SubCommand::with_name("assets").about("Compiles Assets"))

        .get_matches();

    match args.subcommand_name() {
        Some("assets") => {
            println!("Compiling assets...");
            std::fs::create_dir_all("public/assets").ok();

            for file in glob("app/assets/images/*").unwrap() {
                let path = file.unwrap();
                let dest = format!("public/assets/{}", path.file_name().unwrap().to_str().unwrap());
                match fs::copy(path, dest.clone()) {
                    Ok(_) => println!("- copied {}", dest),
                    Err(msg) => println!("- error: {}", msg)
                }
            }

            tasks::stylesheets::compile();
            tasks::javascripts::compile();

        },
        Some(other_task) => println!("Unknown task {}", other_task),
        None => println!("Please specify a task to run")
    }

}
