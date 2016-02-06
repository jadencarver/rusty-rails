use iron::{Request, Response, IronResult};
use iron::mime::Mime;
use iron::status;
use layouts;
use diesel::types::Timestamp;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod views;
use models::entry::Entry;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

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
