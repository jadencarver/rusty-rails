#[derive(Queryable,Clone)]
pub struct Entry {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub public: bool
}

use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;

use params::{Map, Value};

impl Entry {
    pub fn blank() -> Entry {
        Entry {
            id: 0,
            title: "Hello!".to_string(),
            body: "".to_string(),
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
