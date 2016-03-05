use std::fs;

pub fn scaffold(resource: String, fields: Vec<String>) {
    fs::create_dir("app");
    fs::create_dir("app/controllers");
    fs::create_dir("app/models");
    fs::create_dir("app/helpers");
    fs::create_dir(format!("app/controllers/{}", resource));
}
