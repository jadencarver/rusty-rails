extern crate argparse;
extern crate glob;
extern crate serde_json;

use std::io::prelude::*;
use std::fs;
use glob::glob;
use argparse::{ArgumentParser, StoreTrue, Store};
use std::process::{Command};
use std::collections::BTreeMap;

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

            let mut bower_sources: BTreeMap<String, BTreeMap<&str, String>> = BTreeMap::new();
            for file in glob("vendor/assets/*/dist/*.js").unwrap() {
                let path = file.unwrap();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let mut config = BTreeMap::new();
                config.insert("path", path.to_str().unwrap().to_string());
                bower_sources.insert(name, config);
            }
            let mut cjsc_config = fs::File::create("vendor/assets/cjsc.config").unwrap();
            cjsc_config.write_all(serde_json::to_string(&bower_sources).unwrap().as_ref());

            for file in glob("app/assets/*.js").unwrap() {
                let path = file.unwrap();
                let dest_name = path.file_name().unwrap().to_str().unwrap();
                let dest = format!("public/assets/{}", dest_name);
                let source_map = format!("--source-map={}.map", dest);

                Command::new("cjsc").arg("-C").arg("vendor/assets/cjsc.config")
                    .arg(path.clone()).arg("-M")
                    .arg("-o").arg(dest).arg(source_map)
                    .status().unwrap();
            }

            for file in glob("app/assets/*.scss").unwrap() {
                match file {
                    Ok(path) => {
                        let src = path.to_str().unwrap();
                        let dest = format!("public/assets/{}.css", path.file_stem().unwrap().to_str().unwrap());
                        Command::new("scss")
                            .arg(src).arg(dest.clone())
                            .arg("--cache-location").arg("tmp/cache")
                            .arg("--load-path").arg("app/assets/stylesheets")
                            .args(&["--style","compressed"])
                            .status().unwrap();
                        println!("- compiled {}", dest);

                    },
                    Err(e) => println!("{:?}", e),
                }
            }

            for file in glob("app/assets/images/*").unwrap() {
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

        },
        _ => {
            println!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
