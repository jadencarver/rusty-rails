#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(maud_macros, diesel_codegen, dotenv_macros)]

extern crate iron;
#[macro_use] extern crate router;
extern crate logger;
extern crate maud;
extern crate staticfile;
extern crate ansi_term;

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_diesel;

use iron::{Iron, Chain};
use logger::Logger;
use ansi_term::Colour::*;
use dotenv::dotenv;
use std::env;

mod routes;
pub mod controllers;
pub mod models;
pub mod layouts;
pub mod schema;
pub mod helpers;

pub type DBType = diesel::pg::PgConnection;

pub struct DB;
impl iron::typemap::Key for DB { type Value = r2d2::Pool<r2d2_diesel::ConnectionManager<DBType>>; }

fn main() {
    dotenv().ok();
    let hostname = env::var("HOSTNAME")
        .expect("HOSTNAME must be set");
    let mut chain = Chain::new(routes::routes());

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = r2d2_diesel::ConnectionManager::<DBType>::new(database_url);
    let pool = r2d2::Pool::new(r2d2::Config::default(), manager)
        .expect("Database connection failed.");
    chain.link(persistent::Read::<DB>::both(pool));

    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before).link_after(logger_after);

    match Iron::new(chain).http(&hostname[..]) {
        Ok(_) => println!("Started on {}", Green.bold().paint(hostname)),
        Err(error) => println!("Unable to start: {}", error)
    }
}
