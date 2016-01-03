use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status;
use handlebars::Template;
use rustc_serialize::json::{Json, ToJson};
use params::Params;
use router::Router;

pub fn index(request: &mut Request) -> IronResult<Response> {
  Ok(render("index", None))
}

pub fn about(request: &mut Request) -> IronResult<Response> {
  let mut data: BTreeMap<String, Json> = BTreeMap::new();
  data.insert("year".to_string(), "2015".to_json());
  Ok(render("about", Some(data)))
}

fn render(template: &str, data: Option<BTreeMap<String, Json>>) -> Response {
  let mut response = Response::new();
  response.set_mut(Template::new(template, data)).set_mut(status::Ok);
  response
}
