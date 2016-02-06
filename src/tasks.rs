extern crate argparse;
extern crate glob;

use std::fs;
use glob::glob;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::process::{Command};

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

            Command::new("bower").arg("install").status().unwrap();

            for file in glob("vendor/assets/*/dist/*.js").unwrap() {
                match file {
                    Ok(path) => {
                        let dest = format!("public/assets/{}", path.file_name().unwrap().to_str().unwrap());
                        match fs::copy(path, dest.clone()) {
                            Ok(bytes) => println!("- copied {}", dest),
                            Err(msg) => println!("- error: {}", msg)
                        }
                    },
                    Err(e) => println!("{:?}", e),
                }
            }

            for file in glob("app/assets/*.scss").unwrap() {
                match file {
                    Ok(path) => {
                        let src = path.to_str().unwrap();
                        let dest = format!("public/assets/{}.css", path.file_stem().unwrap().to_str().unwrap());
                        Command::new("scss")
                            .arg(src).arg(dest.clone())
                            .args(&["--style","compressed"])
                            .status().unwrap();
                        println!("- compiled {}", dest);

                    },
                    Err(e) => println!("{:?}", e),
                }
            }

    	},
    	_ => {
    		println!("Unknown command: {}", command);
    		std::process::exit(1);
    	}
    }
}