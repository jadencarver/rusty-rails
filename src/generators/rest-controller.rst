use controllers::prelude::*;
use models::{resource}::{Resource};
use schema::{resources}::dsl::{resources};
mod views;

pub fn index(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let results = try!({resources}.get_results::<{Resource}>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::index::index(results))
                      )))
}}

pub fn show(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let {resource} = try!({resource}.find(id).first::<{Resource}>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::show::show({resource}))
                      )))
}}

pub fn new(request: &mut Request) -> IronResult<Response> {{
    let {resource} = {Resource}::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::form::new({resource}, None))
                      )))
}}

pub fn edit(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let {resource} = try!({resource}.find(id).first::<{Resource}>(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::form::edit({resource}, None))
                      )))
}}

pub fn create(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let mut new_{resource} = {Resource}::new();
    new_{resource}.update(params);

    match new_{resource}.is_valid() {{
        Ok(_) => {{
            let {resource}: {Resource} = try!(diesel::insert(&new_{resource}).into({resources}).get_result(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/{resource}/{{}}", new_{resource}.id))),
                               Header(headers::Connection::close())
                              )))
        }},
        Err(errors) => {{
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::{resource}(views::form::new({resource}, errors))
                              )))
        }}
    }}
}}

pub fn update(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let mut {resource} = try!({resources}.find(id).first::<{Resource}>(db));
    {resource}.update(params);

    match {resource}.is_valid() {{
        Ok(_) => {{
            try!({resource}.save_changes::<{Resource}>(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/{resources}/{{}}", {resource}.id))),
                               Header(headers::Connection::close())
                              )))
        }},
        Err(errors)  => {{
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::{resources}(views::form::edit({resource}, errors))
                              )))
        }}
    }}
}}

pub fn delete(request: &mut Request) -> IronResult<Response> {{
     Ok(Response::with((status::Found,
                        Header(headers::Location(format!("/{resource}/{{}}", new_{resource}.id))),
                        Header(headers::Connection::close())
                       )))
}}

