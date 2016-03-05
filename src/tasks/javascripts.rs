use std::process::Command;
use std::collections::BTreeMap;
use glob::glob;
use std::io::prelude::*;
use std::fs::File;
extern crate serde_json;

pub fn compile() {
    Command::new("bower").arg("install").status().expect("Unable to run bower");

    let mut bower_sources: BTreeMap<String, BTreeMap<&str, String>> = BTreeMap::new();
    for path in glob("vendor/assets/*/dist/*.js").unwrap().filter_map(Result::ok) {
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let mut config = BTreeMap::new();
        config.insert("path", path.to_str().unwrap().to_string());
        bower_sources.insert(name, config);
    }
    for path in glob("vendor/assets/*/*.js").unwrap().filter_map(Result::ok) {
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let mut config = BTreeMap::new();
        config.insert("path", path.to_str().unwrap().to_string());
        bower_sources.insert(name, config);
    }

    let mut cjsc_config = File::create("vendor/assets/cjsc.config").unwrap();
    match cjsc_config.write_all(serde_json::to_string(&bower_sources).unwrap().as_ref()) {
        Err(msg) => println!("Error! {}", msg),
        _ => {}
    }

    for file in glob("app/assets/*.js").unwrap() {
        let path = file.unwrap();
        let dest_name = path.file_name().unwrap().to_str().unwrap();
        let dest = format!("public/assets/{}", dest_name);
        let source_map = format!("--source-map={}.map", dest);

        Command::new("cjsc").arg("-C").arg("vendor/assets/cjsc.config")
            .arg(path.clone()) //.arg("-M")
            .arg("-o").arg(dest).arg(source_map)
            .status().unwrap();
    }
}
