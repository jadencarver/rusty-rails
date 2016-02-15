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
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let id = request.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
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
                status::TemporaryRedirect,
                Header(Location("/entries".to_string()))
                )))
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((
                status::Found,
                Header(Location(format!("/entries/{}", entry.id)))
                )))
}

//pub fn delete(request: &mut Request) -> IronResult<Response> {
//    let ref id = request.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
//    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
//    let entry = entries.find(id).first::<Entry>(connection).expect("Error loading entry");
//    Ok(Response::with((
//                status::Ok,
//                "text/html".parse::<Mime>().unwrap(),
//                layouts::application(views::form::edit(entry))
//                )))
//}