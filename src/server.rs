#![feature(plugin)]
#![plugin(maud_macros)]
extern crate iron;
#[macro_use] extern crate router;
extern crate logger;
extern crate maud;
extern crate staticfile;
extern crate postgres;

use iron::{Iron, Request, Response, IronResult, Chain};
use iron::status;
use iron::mime::Mime;
use std::path::Path;

use staticfile::Static;
use logger::Logger;

mod views;

fn homepage(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((
		status::Ok,
		"text/html".parse::<Mime>().unwrap(),
		views::layout(views::homepage())
	)))
}

fn dashboard(_: &mut Request) -> IronResult<Response> {
	Ok(Response::with((
		status::Ok,
		"text/html".parse::<Mime>().unwrap(),
		views::layout(views::dashboard())
	)))
}


fn main() {
	let routes = router!(
		get "/" => homepage,
		get "/_rusty" => dashboard,
		get "/*" => Static::new(Path::new("public"))
	);

	let (logger_before, logger_after) = Logger::new(None);
	let mut chain = Chain::new(routes);
	chain.link_before(logger_before).link_after(logger_after);
	Iron::new(chain)
		.http("0.0.0.0:3000")
		.unwrap();
}
