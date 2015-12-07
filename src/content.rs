use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status;
use hbs::Template;
use rustc_serialize::json::ToJson;

pub fn index(request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();
  let mut data = BTreeMap::new();
  data.insert("year".to_string(), "2015".to_json());
  response.set_mut(Template::new("index", data)).set_mut(status::Ok);
  Ok(response)
}

pub fn about(request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();
  let mut data = BTreeMap::new();
  data.insert("year".to_string(), "2015".to_json());
  response.set_mut(Template::new("about", data)).set_mut(status::Ok);
  Ok(response)
}