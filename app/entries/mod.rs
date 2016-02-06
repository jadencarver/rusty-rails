use iron::{Request, Response, IronResult};
use iron::mime::Mime;
use iron::status;
use layouts;

mod entry;
mod views;

pub fn index(_: &mut Request) -> IronResult<Response> {
  let entries = vec![];
  Ok(Response::with((
    status::Ok,
    "text/html".parse::<Mime>().unwrap(),
    layouts::application(views::index::index(entries))
  )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
  Ok(Response::with((
    status::Ok,
    "text/html".parse::<Mime>().unwrap(),
    layouts::application(views::show::show())
  )))
}