use iron::prelude::*;
use diesel::prelude::*;

use iron::headers;
use iron::modifiers::*;
use iron::status;

use persistent::Read;
use params::{Params, Value};
use router::Router;

use layouts;
use formats;
mod views;

use diesel;
use models::entry::Entry;
use schema::entries::dsl::entries;

const PER_PAGE: i64 = 5;

fn get_entry(request: &Request) -> Entry {
    let pool = request.extensions.get::<Read<::DB>>().unwrap();
    let ref connection = *pool.get().unwrap();

    let params = request.extensions.get::<Router>().unwrap();
    let id = params.find("id").unwrap().parse::<i32>().unwrap();
    entries.find(id).first::<Entry>(connection).expect("Error loading entry")
}

//----

pub fn index(request: &mut Request) -> IronResult<Response> {
    let ref connection = *request.extensions.get::<Read<::DB>>().unwrap().get().unwrap();
    let params = request.get_ref::<Params>().unwrap();
    let page = match params.find(&["p"]).unwrap_or(&Value::Null).clone() {
        Value::String(title) => title.parse::<i64>().unwrap(),
        _ => 0
    };
    let query = entries.limit(PER_PAGE).offset(page*PER_PAGE);
    let index = query.get_results::<Entry>(connection).expect("Error loading entries");
    let num_pages = entries.count().get_result::<i64>(connection).unwrap_or(0) / PER_PAGE;

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::application(views::index::index(index, page, num_pages))
                      )))
}

pub fn new(_: &mut Request) -> IronResult<Response> {
    let entry = Entry::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::application(views::form::new(entry, None))
                      )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::application(views::show::show(entry))
                      )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::application(views::form::edit(entry, None))
                      )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    let params = request.get_ref::<Params>().unwrap().clone();
    let pool = request.extensions.get::<Read<::DB>>().unwrap();
    let ref connection = *pool.get().unwrap();
    let mut entry = Entry::new();
    entry.update(params);

    match entry.is_valid() {
        Ok(_) => {
            let new_entry: Entry = diesel::insert(&entry).into(entries).get_result(connection).unwrap();
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/entries/{}", new_entry.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors) => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::application(views::form::new(entry, errors))
                              )))
        }
    }
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let form_data = request.get_ref::<Params>().unwrap().clone();
    let params = request.extensions.get::<Router>().unwrap();
    let pool = request.extensions.get::<Read<::DB>>().unwrap();
    let ref connection = *pool.get().unwrap();

    let id = params.find("id").unwrap().parse::<i32>().unwrap();

    let mut entry = entries.find(id).first::<Entry>(connection).expect("Error loading entry");
    entry.update(form_data);
    entry.save_changes::<Entry>(&connection).unwrap();

    match entry.is_valid() {
        Ok(_) => {
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/entries/{}", entry.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors)  => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::application(views::form::edit(entry, errors))
                              )))
        }
    }
}

pub fn delete(_: &mut Request) -> IronResult<Response> {
    //let mut entry = get_entry(request);
    //entry.delete();
    Ok(Response::with((
                status::Found,
                Header(headers::Location("/entries/{}".to_string())),
                Header(headers::Connection::close())
                )))
}
