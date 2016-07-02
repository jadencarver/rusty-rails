use controllers::prelude::*;
use schema::entries::dsl::entries;
use models::entry::Entry;
mod views;

pub fn index(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);

    let entries_per_page: i64 = 5;
    let page = match params.find(&["p"]).unwrap_or(&params::Value::Null).clone() {
        params::Value::String(page) => page.parse::<i64>().unwrap(),
        _ => 0
    };

    let ref db = *pool.get().unwrap();
    let query = entries.limit(entries_per_page).offset(page*entries_per_page);
    let index = query.get_results::<Entry>(db).expect("Error loading entries");
    let num_pages = entries.count().get_result::<i64>(db).unwrap_or(0) / entries_per_page;

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::index::index(index, page, num_pages))
                      )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref connection = *pool.get().unwrap();
    let entry = entries.find(id).first::<Entry>(connection).expect("Error loading entry");
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::show::show(entry))
                      )))
}

pub fn new(_: &mut Request) -> IronResult<Response> {
    let entry = Entry::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::form::new(entry, None))
                      )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref connection = *pool.get().unwrap();
    let entry = entries.find(id).first::<Entry>(connection).expect("Error loading entry");
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::entries(views::form::edit(entry, None))
                      )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
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
                               layouts::entries(views::form::new(entry, errors))
                              )))
        }
    }
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let ref connection = *pool.get().unwrap();
    let id = route.find("id").unwrap().parse::<i32>().unwrap();
    let mut entry = entries.find(id).first::<Entry>(connection).expect("Error loading entry");
    entry.update(params);

    match entry.is_valid() {
        Ok(_) => {
            entry.save_changes::<Entry>(connection).unwrap();
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
    let (route, params, pool) = read_request(request);
    //let mut entry = get_entry(request);
    //entry.delete();
    Ok(Response::with((
                status::Found,
                Header(headers::Location("/entries/{}".to_string())),
                Header(headers::Connection::close())
                )))
}
