use iron::prelude::*;
use diesel::prelude::*;

use iron::headers;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::*;
use iron::status;

use persistent::Read;
use params::Params;
use router::Router;

use layouts;
mod views;

use diesel;
use models::entry::{NewEntry, Entry};
use schema::entries::dsl::entries;

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
    let index = entries.limit(5).load::<Entry>(connection).expect("Error loading entries");

    Ok(Response::with((status::Ok,
                Header(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))),
                layouts::application(views::index::index(index))
                )))
}

pub fn new(_: &mut Request) -> IronResult<Response> {
    let entry = Entry::new();
    Ok(Response::with((status::Ok,
                Header(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))),
                layouts::application(views::form::new(entry, None))
                )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                Header(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))),
                layouts::application(views::show::show(entry))
                )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let entry = get_entry(request);
    Ok(Response::with((status::Ok,
                Header(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))),
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
        Ok(entry_id) => {
            let new_entry: Entry = diesel::insert(&entry).into(entries).get_result(connection).unwrap();
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/entries/{}", new_entry.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors) => Ok(Response::with((status::NotAcceptable,
                                          Header(headers::ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))),
                                          layouts::application(views::form::new(entry, errors))
                                         )))
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
    entry.save_changes::<Entry>(&connection);

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
    let mut entry = get_entry(request);
    //entry.delete();
    Ok(Response::with((
                status::Found,
                Header(headers::Location("/entries/{}".to_string())),
                Header(headers::Connection::close())
                )))
}
