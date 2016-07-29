use chrono;
use std::fmt::Display;
use clap::Values;

pub struct Resource {
    pub name: String,
    pub plural: String,
    pub constant: String,
    pub timestamp: String
}

fn pluralize<T: Display>(text: T) -> String {
	let orig_string = format!("{}", text);
	let string = orig_string.trim();
	if string == "" { return String::new() }
	let string_lowercase = string.to_lowercase();
	let string_last_char = string_lowercase.chars().last().unwrap();
	let string_second_to_last_char = string_lowercase.chars().rev().nth(1).unwrap_or(' ');
	match string_last_char {
		'y' => match string_second_to_last_char {
			'a' | 'e' | 'i' | 'o' | 'u' => format!("{}s", string),
			_ => format!("{}ies", &string[0..string.len()-1])
		},
		'h' if string_second_to_last_char == 'c' || string_second_to_last_char == 's' => format!("{}es", string),
		'x' | 's' | 'z' | 'o' => format!("{}es", string),
		_ => match string_lowercase.as_ref() {
			"goose" => format!("{:.1}eese", string),
			"knife" | "loaf" => format!("{:.3}ves", string),
			"leaf" => format!("{:.3}ves", string),
			"deer" => format!("{}", string),
			_ => format!("{}s", string)
		}
	}
}

impl Resource {
    pub fn new(resource: &str) -> Resource {
        Resource {
            name: String::from(resource),
            plural: pluralize(resource),
            constant: format!("{}{}", &resource.to_uppercase()[0..1], &resource[1..]),
            timestamp: format!("{}", chrono::Local::now().format("%Y%m%d%H%M%S"))
        }
    }
}

pub struct Field {
    pub field_pub: bool,
    pub field_name: String,
    pub field_type: String
}

pub enum FieldType {
    // supported concrete types:
    String(i32), Text(i32),
    Symbol, Boolean, Integer, Decimal, Float, DateTime, Date,
    // supported abstract types:
    Color, Image, Video, File, Url,
    Phone, Email, Password, Search
}

impl Field {

    pub fn parse(attributes: Values) -> Vec<Field> {
        attributes.map( |attribute| {
            let mut split = attribute.split(':');
            let attr1 = split.next().unwrap_or("");
            let attr2 = split.next().unwrap_or("");
            let attr3 = split.next().unwrap_or("");
            if attr1 == "pub" && attr2 != "" && attr3 != "" {
                Field { // pub:name:type
                    field_pub: true,
                    field_name: String::from(attr2),
                    field_type: String::from(attr3)
                }
            } else if attr1 == "pub" && attr2 != "" && attr3 == "" {
                Field { // pub:name
                    field_pub: true,
                    field_name: String::from(attr2),
                    field_type: String::from(attr2)
                }
            } else if attr1 != "" && attr1 != "pub" && attr2 != "" && attr3 == "" {
                Field { // name:type
                    field_pub: false,
                    field_name: String::from(attr1),
                    field_type: String::from(attr2)
                }
            } else if attr1 != "" && attr2 == "" && attr3 == "" {
                Field { // type
                    field_pub: true,
                    field_name: String::from(attr1),
                    field_type: String::from(attr1)
                }
            } else { panic!("Unable to interpret field arguments!") }
        }).collect()
    }

    // converts to strict type from plain-english types
    fn general_type(&self) -> Option<FieldType> {
        match self.field_type.as_ref() {
            "bool" | "boolean" => Some(FieldType::Boolean),
            "str" | "string" | "title" => Some(FieldType::String(255)),
            "sym" | "city" | "state" | "zip" => Some(FieldType::Symbol),
            "text" | "description" | "summary" | "content" => Some(FieldType::Text(255)),
            "decimal" => Some(FieldType::Decimal),
            "email" => Some(FieldType::Email),
            "url" => Some(FieldType::Url),
            "image" | "picture" => Some(FieldType::Image),
            "file" => Some(FieldType::File),
            "date" => Some(FieldType::Date),
            "datetime" | "timestamp" => Some(FieldType::DateTime),
            _ => None
        }
    }

	pub fn rust_type(&self) -> String {
        let general_type = self.general_type().expect(&format!("type could not be determined for {}", self.field_type));
		let rust_type = match general_type {
			FieldType::Boolean => "Boolean",
			FieldType::String(_) | FieldType::Text(_) | FieldType::Symbol | FieldType::Email | FieldType::Url | FieldType::Color | FieldType::Image | FieldType::Video | FieldType::File | FieldType::Phone | FieldType::Password | FieldType::Search => "String",
			FieldType::Integer => "i64",
			FieldType::Decimal => "Decimal",
			FieldType::Float => "f64",
			FieldType::DateTime | FieldType::Date => "Timestamp",
		};
		match self.field_pub {
			true => rust_type.to_string(),
			false => format!("Option<{}>", rust_type)
		}
	}

    // returns the SQL appropriate column type
    pub fn sql_type(&self) -> String {
        let general_type = self.general_type().expect(&format!("type could not be determined for {}", self.field_type));
        let sql_type = format!(",\n    {} {}{}", self.field_name, match general_type {
            FieldType::Boolean => format!("BOOLEAN"),
            FieldType::String(len) => format!("VARCHAR({})", len),
            FieldType::Text(_) => format!("TEXT"),
            FieldType::Symbol | FieldType::Phone | FieldType::Color => format!("VARCHAR(32)"),
            FieldType::File | FieldType::Image | FieldType::Video | FieldType::Email | FieldType::Password | FieldType::Search | FieldType::Url => format!("VARCHAR(255)"),
            FieldType::Integer | FieldType::Decimal | FieldType::Float => format!("INTEGER"),
            FieldType::DateTime | FieldType::Date => format!("TIMESTAMP WITH TIME ZONE"),
        }, match self.field_pub {
			true => " NOT NULL",
			false => ""
        });
        sql_type
    }

}

pub mod scaffold;
