use controllers::prelude::*;
use models::entry::*;
use schema::entries::dsl::entries;

mod views {
    pub mod form;
    pub mod index;
    pub mod show;
}

pub fn index(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let results = itry!(entries.get_results::<Entry>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::index::index(results))
                      )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = itry!(route.find("id").unwrap_or("").parse::<i32>(), (status::BadRequest));
    let ref db = *pool.get().unwrap();
    let entry = match entries.find(id).first::<Entry>(db) {
        Ok(entry) => entry,
        Err(error) => match error {
            diesel::result::Error::NotFound => return Err(IronError::new(error, (status::NotFound))),
            _ => return Err(IronError::new(error, (status::InternalServerError)))
        }
    };

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::show::show(entry))
                      )))
}

pub fn new(request: &mut Request) -> IronResult<Response> {
    let entry = Entry::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::form::new(entry, None))
                      )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = itry!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let entry = itry!(entries.find(id).first::<Entry>(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::form::edit(entry, None))
                      )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let mut new_entry = Entry::new();
    new_entry.update(params);

    match new_entry.is_valid() {
        Ok(_) => {
            let entry: Entry = itry!(diesel::insert(&new_entry).into(entries).get_result(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/entries/{}", entry.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors) => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::entries(views::form::new(new_entry, errors))
                              )))
        }
    }
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = itry!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let mut entry = itry!(entries.find(id).first::<Entry>(db));
    entry.update(params);

    match entry.is_valid() {
        Ok(_) => {
            itry!(entry.save_changes::<Entry>(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/entries/{}", entry.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors)  => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::entries(views::form::edit(entry, errors))
                              )))
        }
    }
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    let (route, _params, pool) = read_request(request);
    let id = itry!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let mut entry = itry!(entries.find(id).first::<Entry>(db));
    Ok(Response::with((status::Found,
                       Header(headers::Location(format!("/entry/{}", entry.id))),
                       Header(headers::Connection::close())
                      )))
}

