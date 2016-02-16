use iron::{Request, Response, IronResult};
use iron::status;
use iron::modifiers::Header;
use iron::headers::{Location};
use iron::mime::Mime;
use router::Router;
use layouts;

use diesel::prelude::*;

mod views;
use models::entry::Entry;
use schema::entries::dsl::entries;
use persistent::Read;

fn get_entry(request: &Request) -> Entry {
    let pool = request.extensions.get::<Read<::DB>>().unwrap();
    let ref connection = *pool.get().unwrap();

    let params = request.extensions.get::<Router>().unwrap();
    let id = params.find("id").unwrap().parse::<i32>().unwrap();
    entries.find(id).first::<Entry>(connection).expect("Error loading entry")
}

pub fn index(request: &mut Request) -> IronResult<Response> {
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let index = entries.limit(5).load::<Entry>(connection).expect("Error loading entries");

    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::index::index(index))
                )))
}

pub fn new(request: &mut Request) -> IronResult<Response> {
    let entry = Entry::blank();
    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::form::new(entry))
                )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::show::show(entry))
                )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::form::edit(entry))
                )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
                status::Found,
                Header(Location("/entries".to_string())),
                "THIS IS A HEY HEY HEY!"
                )))
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Found,
                "text/html".parse::<Mime>().unwrap(),
                Header(Location(format!("http://localhost:3000/entries/{}", entry.id))),
                "THIS IS A HEY HEY HEY!"
                )))
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Found,
                "text/html".parse::<Mime>().unwrap(),
                Header(Location(format!("http://localhost:3000/entries/{}", entry.id))),
                "THIS IS A HEY HEY HEY!"
                )))
}
