use std::error::Error;
use clap::{ArgMatches, Values};

mod scaffold;
mod controller;
mod model;
mod migration;
mod view;
pub use generators::scaffold::Scaffold;
pub use generators::controller::Controller;
pub use generators::model::Model;
pub use generators::migration::Migration;
pub use generators::view::View;

#[derive(Debug)]
pub struct GeneratorError {
    pub description: String,
    pub cause: Option<&'static Error>
}

#[derive(Clone)]
enum LiteralType {
    // concrete types:
    String(i32), Text(i32),
    Boolean, Integer, Decimal, Float, DateTime, Date,
    // abstract types:
    Symbol, Color, Image, Video, File, Url,
    Phone, Email, Password, Search
}

#[derive(Clone)]
enum Access {
    Required, Optional, Private
}

#[derive(Clone)]
struct Field {
    name: String,
    access: Access,
    literal: LiteralType
}

#[derive(Clone)]
pub struct Resource {
    name: Noun,
    fields: Vec<Field>
}

pub trait Generator {
    fn new(resource: Resource) -> Result<Box<Self>, GeneratorError>;
    fn generate(self);
}

impl Resource {
    pub fn from(args: &ArgMatches) -> Result<Resource, GeneratorError> {
        Ok(Resource {
            name: Noun(String::from(args.value_of("resource").unwrap())),
            fields: try!(Field::from(args.values_of("fields").unwrap()))
        })
    }
}

impl Field {
    pub fn from(args: Values) -> Result<Vec<Field>, GeneratorError> {
        let mut fields: Vec<Field> = vec![];
        for attribute in args {
            let attribute = attribute.to_lowercase();
            let mut split = attribute.split(':');
            let attr1 = split.next().unwrap_or("");
            let attr2 = split.next().unwrap_or("");
            let attr3 = split.next().unwrap_or("");
            if attr1 == "pub" && attr2 != "" && attr3 != "" {
                fields.push(Field { // pub:name:type
                    access: Access::Required,
                    name: String::from(attr2),
                    literal: try!(LiteralType::from(attr3))
                })
            } else if attr1 == "pub" && attr2 != "" && attr3 == "" {
                fields.push(Field { // pub:name
                    access: Access::Required,
                    name: String::from(attr2),
                    literal: try!(LiteralType::from(attr2))
                })
            } else if attr1 == "opt" && attr2 != "" && attr3 != "" {
                fields.push(Field { // opt:name:type
                    access: Access::Optional,
                    name: String::from(attr2),
                    literal: try!(LiteralType::from(attr3))
                })
            } else if attr1 == "pub" && attr2 != "" && attr3 == "" {
                fields.push(Field { // opt:name
                    access: Access::Optional,
                    name: String::from(attr2),
                    literal: try!(LiteralType::from(attr2))
                })
            } else if attr1 != "" && attr1 != "pub" && attr2 != "" && attr3 == "" {
                fields.push(Field { // name:type
                    access: Access::Private,
                    name: String::from(attr1),
                    literal: try!(LiteralType::from(attr2))
                })
            } else if attr1 != "" && attr2 == "" && attr3 == "" {
                fields.push(Field { // type
                    access: Access::Required,
                    name: String::from(attr1),
                    literal: try!(LiteralType::from(attr1))
                })
            } else {
                return Err(GeneratorError {
                    description: String::from("Unable to interpret field arguments"), cause: None
                })
            }
        }
        Ok(fields)
    }
}

impl LiteralType {
    pub fn from(type_str: &str) -> Result<LiteralType, GeneratorError> {
        match type_str {
            "bool" | "boolean" => Ok(LiteralType::Boolean),
            "str" | "string" | "name" | "title" => Ok(LiteralType::String(255)),
            "search" => Ok(LiteralType::Search),
            "symbol" | "sym" | "city" | "state" | "zip" => Ok(LiteralType::Symbol),
            "text" | "description" | "summary" | "content" => Ok(LiteralType::Text(255)),
            "integer" | "int" => Ok(LiteralType::Integer),
            "float" => Ok(LiteralType::Float),
            "decimal" => Ok(LiteralType::Decimal),
            "phone" | "tel" => Ok(LiteralType::Phone),
            "email" => Ok(LiteralType::Email),
            "color" => Ok(LiteralType::Color),
            "url" => Ok(LiteralType::Url),
            "image" | "picture" => Ok(LiteralType::Image),
            "video" => Ok(LiteralType::Video),
            "file" => Ok(LiteralType::File),
            "date" => Ok(LiteralType::Date),
            "datetime" | "timestamp" => Ok(LiteralType::DateTime),
            "password" => Ok(LiteralType::Password),
            _ => Err(GeneratorError {
                description: String::from("Unrecognized field type"), cause: None
            })
        }
    }
}

#[derive(Clone)]
struct Noun(String);

impl Noun {
    fn plural(&self) -> String {
        let noun = &self.0.trim();
        if *noun == "" { return String::new() }
        let noun_lowercase = noun.to_lowercase();
        let noun_last_char = noun_lowercase.chars().last().unwrap();
        let noun_second_to_last_char = noun_lowercase.chars().rev().nth(1).unwrap_or(' ');
        match noun_last_char {
            'y' => match noun_second_to_last_char {
                'a' | 'e' | 'i' | 'o' | 'u' => format!("{}s", noun),
                _ => format!("{}ies", &noun[0..noun.len()-1])
            },
            'h' if noun_second_to_last_char == 'c' || noun_second_to_last_char == 's' => format!("{}es", noun),
            'x' | 's' | 'z' | 'o' => format!("{}es", noun),
            _ => match noun_lowercase.as_ref() {
                "goose" => format!("{:.1}eese", noun),
                "knife" | "loaf" => format!("{:.3}ves", noun),
                "leaf" => format!("{:.3}ves", noun),
                "deer" => format!("{}", noun),
                _ => format!("{}s", noun)
            }
        }
    }
}
