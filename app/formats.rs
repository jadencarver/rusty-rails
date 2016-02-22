use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub fn html() -> ContentType {
    ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)]))
}
