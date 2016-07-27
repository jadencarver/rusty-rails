use chrono;
use std::fmt::Display;

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
        }, if self.field_pub {" NOT NULL"} else {""} );
        print!("{}", sql_type);
        sql_type
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
}

pub mod scaffold;
