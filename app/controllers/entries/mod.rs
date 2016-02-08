use iron::{Request, Response, IronResult};
use router::Router;
use iron::mime::Mime;
use iron::status;
use layouts;

use diesel::prelude::*;

mod views;
use models::entry::Entry;
use schema::entries::dsl::entries as Entries;
use helpers::db::establish_connection;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let connection = establish_connection();
    let entries = Entries.limit(5).load::<Entry>(&connection).expect("Error loading entries");

    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::index::index(entries))
                )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let ref id = request.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
    let connection = establish_connection();
    let entry = Entries.find(id).first::<Entry>(&connection).expect("Error loading entry");
    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::show::show(entry))
                )))
}
