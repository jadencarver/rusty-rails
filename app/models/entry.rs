use diesel::prelude::*;
use schema::entries;
use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;
use params::{Map, Value};

#[derive(Queryable)]
#[insertable_into(entries)]
#[changeset_for(entries)]
pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool
}

#[insertable_into(entries)]
pub struct NewEntry {
  pub title: String,
  pub body: String,
  pub public: bool
}

impl NewEntry {
    pub fn to_generic(self) -> Entry {
        Entry {
            id: 0,
            title: self.title,
            body: self.body,
            public: self.public
        }
    }

    pub fn update(&mut self, params: Map) {
        match params.find(&["entry","title"]).unwrap().clone() {
            Value::String(title) => self.title = title,
            _ => {}
        }
        match params.find(&["entry","body"]).unwrap().clone() {
            Value::String(body) => self.body = body,
            _ => {}
        }
    }

    pub fn is_valid(&mut self) -> Result<i32, Errors> {
        let mut errors = HashMap::new();

        if self.title.is_empty() { errors.insert("title", vec!["can't be blank"]); }
        if self.body.is_empty()  { errors.insert("body", vec!["can't be blank"]); }

        if errors.is_empty() {
            Ok(0)
        } else {
            Err(Some(errors))
        }
    }
}

impl Entry {
    pub fn new() -> NewEntry {
        NewEntry {
            title: String::new(),
            body: String::new(),
            public: false
        }
    }

    pub fn update(&mut self, params: Map) {
        match params.find(&["entry","title"]).unwrap().clone() {
            Value::String(title) => self.title = title,
            _ => {}
        }
        match params.find(&["entry","body"]).unwrap().clone() {
            Value::String(body) => self.body = body,
            _ => {}
        }
    }

    pub fn is_valid(&mut self) -> Result<i32, Errors> {
        let mut errors = HashMap::new();

        if self.title.is_empty() { errors.insert("title", vec!["can't be blank"]); }
        if self.body.is_empty()  { errors.insert("body", vec!["can't be blank"]); }

        if errors.is_empty() {
            Ok(self.id)
        } else {
            Err(Some(errors))
        }
    }

}
