extern crate argparse;
extern crate glob;

use std::fs;
use argparse::{ArgumentParser, StoreTrue, Store};
use glob::glob;

mod tasks;

fn main() {
    let mut verbose = false;
    let mut command = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Runs tasks related to managing a Rusty Rails environment");
        ap.refer(&mut command).add_argument("command", Store, "Command to Run");
        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Verbose output");
        ap.parse_args_or_exit();
    }

    if verbose { println!("Running command {}", command); }
    match command.as_ref() {
        "assets" => {

            println!("Compiling assets...");
            std::fs::create_dir_all("public/assets").ok();

            tasks::javascripts::compile();
            tasks::stylesheets::compile();

            for file in glob("app/assets/images/*").unwrap() {
                let path = file.unwrap();
                let dest = format!("public/assets/{}", path.file_name().unwrap().to_str().unwrap());
                match fs::copy(path, dest.clone()) {
                    Ok(_) => println!("- copied {}", dest),
                    Err(msg) => println!("- error: {}", msg)
                }
            }

        },
        _ => {
            println!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
