use iron::{Request, Response, IronResult};
use router::Router;
use iron::mime::Mime;
use iron::status;
use layouts;

use diesel::prelude::*;

mod views;
use models::entry::Entry;
use schema::entries::dsl::entries as Entries;
use persistent::Read;

pub fn index(request: &mut Request) -> IronResult<Response> {
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let entries = Entries.limit(5).load::<Entry>(connection).expect("Error loading entries");

    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::index::index(entries))
                )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let ref id = request.extensions.get::<Router>().unwrap().find("id").unwrap().parse::<i32>().unwrap();
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let entry = Entries.find(id).first::<Entry>(connection).expect("Error loading entry");
    Ok(Response::with((
                status::Ok,
                "text/html".parse::<Mime>().unwrap(),
                layouts::application(views::show::show(entry))
                )))
}
