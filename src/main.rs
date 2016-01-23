#![feature(plugin)]
#![plugin(maud_macros)]
extern crate iron;
extern crate postgres;
#[macro_use]
extern crate router;
extern crate logger;
extern crate maud;

use iron::{Iron, Request, Response, IronResult, Chain};
use iron::status;
use iron::mime::Mime;
use router::{Router};
use std::fmt::Write;
use logger::Logger;

mod views;

fn homepage(req: &mut Request) -> IronResult<Response> {
	let mut body = String::new();
	html!(body, {
		p { "Hello" }
	});
	Ok(Response::with((
		"text/html".parse::<Mime>().unwrap(),
		status::Ok,
		views::layout(body)
	)))
}

fn main() {
	let (logger_before, logger_after) = Logger::new(None);
	let mut chain = Chain::new(router!(
		get "/" => homepage,
		get "/style.css" => stylesheets
	));
	chain.link_before(logger_before);
	chain.link_after(logger_after);
	Iron::new(chain)
		.http("localhost:3000")
		.unwrap();
}