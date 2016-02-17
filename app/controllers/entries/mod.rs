use iron::prelude::*;
use diesel::prelude::*;
use diesel;

use iron::status;
use iron::modifiers::Header;
use iron::headers;
use iron::mime::Mime;
use router::Router;
use layouts;
use params::{Params, Value};

mod views;
use models::entry::Entry;
use schema::entries::dsl::entries;
use persistent::Read;

pub fn index(request: &mut Request) -> IronResult<Response> {
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let index = entries.limit(5).load::<Entry>(connection).expect("Error loading entries");

    Ok(Response::with((status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::index::index(index))
                )))
}

pub fn new(request: &mut Request) -> IronResult<Response> {
    let entry = Entry::blank();
    Ok(Response::with((status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::form::new(entry, None))
                )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::show::show(entry))
                )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::form::edit(entry, None))
                )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Found,
                Header(headers::Location("/entries".to_string())),
                Header(headers::Connection::close())
                )))
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let mut entry = get_entry(request);
    let params = request.get_ref::<Params>().unwrap();
    match params.find(&["entry","title"]).unwrap().clone() {
        Value::String(title) => entry.title = title,
        _ => println!("invalid argument")
    }
    //entry.body = params.find(&["entry","body"]).unwrap();
    match entry.is_valid() {
        Ok(entry_id) => Ok(Response::with((status::Found,
                           Header(headers::Location(format!("/entries/{}", entry_id))),
                           Header(headers::Connection::close())
                        ))),
        Err(errors)  => Ok(Response::with((status::NotAcceptable,
                           "text/html".parse::<Mime>().unwrap(),
                           layouts::application(views::form::edit(entry, errors))
                        )))
    }
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Found,
                Header(headers::Location("/entries/{}".to_string())),
                Header(headers::Connection::close())
                )))
}

// --- private methods
//
fn get_entry(request: &Request) -> Entry {
    let pool = request.extensions.get::<Read<::DB>>().unwrap();
    let ref connection = *pool.get().unwrap();

    let params = request.extensions.get::<Router>().unwrap();
    let id = params.find("id").unwrap().parse::<i32>().unwrap();
    entries.find(id).first::<Entry>(connection).expect("Error loading entry")
}


