use iron::{Request, Response, IronResult};
use iron::mime::Mime;
use iron::status;
use layouts;
use diesel::types::Timestamp;

mod entry;
mod views;
use entries::entry::Entry;

pub fn index(_: &mut Request) -> IronResult<Response> {
  let entries = vec![
    Entry {
      id: 0,
      title: "Hello World".to_string(),
      body: "Lorem ipsum dolor sit amit.".to_string(),
      public: true,
      created_at: Timestamp::default()
    },
    Entry {
      id: 0,
      title: "Another Entry".to_string(),
      body: "Lorem ipsum dolor sit amit.".to_string(),
      public: true,
      created_at: Timestamp::default()
    }
  ];

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