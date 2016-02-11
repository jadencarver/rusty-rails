#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(maud_macros, diesel_codegen, dotenv_macros)]

extern crate iron;
#[macro_use] extern crate router;
extern crate logger;
extern crate maud;
extern crate staticfile;
extern crate ansi_term;

extern crate dotenv;
#[macro_use] extern crate diesel;

use iron::{Iron, Chain};
use logger::Logger;
use ansi_term::Colour::*;

mod routes;
pub mod controllers;
pub mod models;
pub mod layouts;
pub mod schema;
pub mod helpers;

fn main() {
    let hostname = "0.0.0.0:3000";

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(routes::routes());
    chain.link_before(logger_before).link_after(logger_after);

    match Iron::new(chain).http(hostname) {
        Ok(_) => println!("Started on {}", Green.bold().paint(hostname)),
        Err(error) => println!("Unable to start: {}", error)
    }
}
