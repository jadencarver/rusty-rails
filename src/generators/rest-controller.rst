use controllers::prelude::*;
use models::{resource}::{Resource};
use schema::{resources}::dsl::{resources};
mod views;

pub fn index(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
    let results = try!({resources}.get_results::<{Resource}>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::index::index(results))
                      )))
}}

pub fn show(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = params(request);
    let id = try!(try!(route.find("id")).parse::<i32>());
    let ref db = try!(*pool.get());
    let {resource} = try!({resource}.find(id).first::<{Resource}>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::show::show())
                      )))
}}

pub fn new(request: &mut Request) -> IronResult<Response> {{
    let {resource} = {Resource}::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::form::new({resource}))
                      )))
}}

pub fn edit(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = params(request);
    let id = try!(try!(route.find("id")).parse::<i32>());
    let ref db = try!(*pool.get());
    let {resource} = try!({resource}.find(id).first::<{Resource}>(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resource}(views::form::edit())
                      )))
}}

pub fn create(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
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
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
    let id = try!(try!(route.find("id")).parse::<i32>());
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

