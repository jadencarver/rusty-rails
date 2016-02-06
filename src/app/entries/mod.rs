use iron::{Request, Response, IronResult};
use iron::mime::Mime;
use iron::status;
use app;

mod entry;
mod views;

pub fn index(_: &mut Request) -> IronResult<Response> {
  let entries = vec![];
  Ok(Response::with((
    status::Ok,
    "text/html".parse::<Mime>().unwrap(),
    app::layout(views::index::index(entries))
  )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
  Ok(Response::with((
    status::Ok,
    "text/html".parse::<Mime>().unwrap(),
    app::layout(views::show::show())
  )))
}