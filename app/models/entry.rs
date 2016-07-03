use diesel::prelude::*;
use schema::entries;
use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;
use params::{Map, Value};

pub trait EntryModel {
    fn title(&self) -> &String;
    fn body(&self) -> &String;
    fn set_title(&mut self, title: String);
    fn set_body(&mut self, title: String);
    fn update(&mut self, params: Map);
    fn is_valid(&mut self) -> Result<bool, Errors>;
}

fn validate<Entry: EntryModel>(entry: &Entry) -> Result<bool, Errors> {
    let mut errors = HashMap::new();

    if entry.title().is_empty() { errors.insert("title", vec!["can't be blank"]); }
    if entry.body().is_empty()  { errors.insert("body", vec!["can't be blank"]); }

    if errors.is_empty() {
        Ok(true)
    } else {
        Err(Some(errors))
    }
}

fn update<Entry: EntryModel>(entry: &mut Entry, params: Map) {
    match params.find(&["entry","title"]).unwrap().clone() {
        Value::String(title) => entry.set_title(title),
        _ => {}
    }
    match params.find(&["entry","body"]).unwrap().clone() {
        Value::String(body) => entry.set_body(body),
        _ => {}
    }
}

//

#[derive(Queryable)]
#[insertable_into(entries)]
#[changeset_for(entries)]
pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool
}

impl Entry {
    pub fn new() -> NewEntry {
        NewEntry {
            title: String::new(),
            body: String::new(),
            public: false
        }
    }
}

impl EntryModel for Entry {
    fn title(&self) -> &String { &self.title }
    fn body (&self) -> &String { &self.body  }
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn set_title(&mut self, title: String) { self.title = title }
    fn set_body(&mut self, body: String) { self.body = body }
}

#[insertable_into(entries)]
pub struct NewEntry {
  pub title: String,
  pub body: String,
  pub public: bool
}

impl EntryModel for NewEntry {
    fn title(&self) -> &String { &self.title }
    fn body (&self) -> &String { &self.body  }
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn set_title(&mut self, title: String) { self.title = title }
    fn set_body(&mut self, body: String) { self.body = body }
}

