#![feature(plugin)]
#![plugin(maud_macros)]
#![feature(custom_derive, custom_attribute, plugin)]
#[macro_use]
extern crate diesel;
// #![plugin(diesel_codegen, dotenv_macros)]

extern crate dotenv;
extern crate iron;
#[macro_use] extern crate router;
extern crate logger;
extern crate maud;
extern crate staticfile;
extern crate postgres;

// pub mod schema;
// pub mod models;

use iron::{Iron, Chain};
use std::path::Path;
use staticfile::Static;
use logger::Logger;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod layouts;
mod pages;
mod entries;

pub fn establish_connection() -> PgConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
      .expect(&format!("Error connecting to {}", database_url))
}

fn main() {

	let routes = router!(
		get "/" => pages::index,
		get "/entries" => entries::index,
		get "/entries/:id" => entries::show,
		get "/*" => Static::new(Path::new("public"))
	);

	let (logger_before, logger_after) = Logger::new(None);
	let mut chain = Chain::new(routes);
	chain.link_before(logger_before).link_after(logger_after);

	Iron::new(chain)
		.http("0.0.0.0:3000")
		.unwrap();
}
