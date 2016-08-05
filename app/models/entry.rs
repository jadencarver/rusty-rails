use diesel::prelude::*;
use schema::entries;
use std::fmt;

use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;
use params::{Map, Value};

pub trait EntryModel {
    fn update(&mut self, params: Map);
    fn is_valid(&mut self) -> Result<bool, Errors>;
    fn title(&self) -> &String;
    fn set_title(&mut self, title: String);
    fn description(&self) -> &String;
    fn set_description(&mut self, description: String);
}

fn validate<Entry: EntryModel>(entry: &Entry) -> Result<bool, Errors> {
    let mut errors = HashMap::new();

    if entry.title().is_empty() { errors.insert("title", vec!["can't be blank"]); }
    if entry.description().is_empty() { errors.insert("description", vec!["can't be blank"]); }

    if errors.is_empty() {
        Ok(true)
    } else {
        Err(Some(errors))
    }
}

fn update<Entry: EntryModel>(entry: &mut Entry, params: Map) {
    match params.find(&["entry","title"]).unwrap().clone() {
        Value::String(title) => entry.set_title(title), _ => {}
    }
    match params.find(&["entry","description"]).unwrap().clone() {
        Value::String(description) => entry.set_description(description), _ => {}
    }
}

#[derive(Queryable)]
#[insertable_into(entries)]
#[changeset_for(entries)]
pub struct Entry {
    pub id: i32, 
    pub title: String,
    pub description: String,
}

impl Entry {
    pub fn new() -> NewEntry {
        NewEntry {
            title: String::new(),
            description: String::new(),
        }
    }
    pub fn id(&self) -> i32 { self.id }
}

impl EntryModel for Entry {
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn title(&self) -> &String { &self.title }
    fn set_title(&mut self, title: String) { self.title = title }
    fn description(&self) -> &String { &self.description }
    fn set_description(&mut self, description: String) { self.description = description }
}

impl fmt::Display for Entry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("Entry #{}", self.id))
    }
}

#[insertable_into(entries)]
pub struct NewEntry {
    pub title: String,
    pub description: String,
}

impl EntryModel for NewEntry {
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn title(&self) -> &String { &self.title }
    fn set_title(&mut self, title: String) { self.title = title }
    fn description(&self) -> &String { &self.description }
    fn set_description(&mut self, description: String) { self.description = description }
}

