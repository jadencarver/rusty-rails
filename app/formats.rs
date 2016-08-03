use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub fn html() -> ContentType {
    ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)]))
}

pub fn xml() -> ContentType {
    ContentType(Mime(TopLevel::Application, SubLevel::Xml, vec![(Attr::Charset, Value::Utf8)]))
}

pub fn json() -> ContentType {
    ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]))
}
