use diesel::prelude::*;
use schema::pictures;
use std::fmt;

use std::collections::HashMap;
pub type Errors = Option<HashMap<&'static str, Vec<&'static str>>>;
use params::{Map, Value};

pub trait PictureModel {
    fn update(&mut self, params: Map);
    fn is_valid(&mut self) -> Result<bool, Errors>;
    fn title(&self) -> &String;
    fn set_title(&mut self, title: String);
}

fn validate<Picture: PictureModel>(picture: &Picture) -> Result<bool, Errors> {
    let mut errors = HashMap::new();

    if picture.title().is_empty() { errors.insert("title", vec!["can't be blank"]); }

    if errors.is_empty() {
        Ok(true)
    } else {
        Err(Some(errors))
    }
}

fn update<Picture: PictureModel>(picture: &mut Picture, params: Map) {
    match params.find(&["picture","title"]).unwrap().clone() {
        Value::String(title) => picture.set_title(title), _ => {}
    }
}

#[derive(Queryable)]
#[insertable_into(pictures)]
#[changeset_for(pictures)]
pub struct Picture {
    pub id: i32, 
    pub title: String,
}

impl Picture {
    pub fn new() -> NewPicture {
        NewPicture {
            title: String::new(),
        }
    }
}

impl PictureModel for Picture {
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn title(&self) -> &String { &self.title }
    fn set_title(&mut self, title: String) { self.title = title }
}

impl fmt::Display for Picture {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str(&format!("Picture #{}", self.id))
    }
}

#[insertable_into(pictures)]
pub struct NewPicture {
    pub title: String,
}

impl PictureModel for NewPicture {
    fn update(&mut self, params: Map) { update(self, params) }
    fn is_valid(&mut self) -> Result<bool, Errors> { validate(self) }
    fn title(&self) -> &String { &self.title }
    fn set_title(&mut self, title: String) { self.title = title }
}

