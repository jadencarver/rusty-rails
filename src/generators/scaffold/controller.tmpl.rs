use controllers::prelude::*;
use models::{resource}::*;
use schema::{resources}::dsl::{resources};

mod views {{
    pub mod form;
    pub mod index;
    pub mod show;
}}

pub fn index(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let results = {resources}.get_results::<{Resource}>(db).unwrap();

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::index::index(results))
                      )))
}}

pub fn show(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref db = *pool.get().unwrap();
    let {resource} = {resources}.find(id).first::<{Resource}>(db).unwrap();

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::show::show({resource}))
                      )))
}}

pub fn new(request: &mut Request) -> IronResult<Response> {{
    let {resource} = {Resource}::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::form::new({resource}, None))
                      )))
}}

pub fn edit(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref db = *pool.get().unwrap();
    let {resource} = {resources}.find(id).first::<{Resource}>(db).unwrap();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::form::edit({resource}, None))
                      )))
}}

pub fn create(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let mut new_{resource} = {Resource}::new();
    new_{resource}.update(params);

    match new_{resource}.is_valid() {{
        Ok(_) => {{
            let {resource}: {Resource} = diesel::insert(&new_{resource}).into({resources}).get_result(db).unwrap();
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/{resources}/{{}}", {resource}.id))),
                               Header(headers::Connection::close())
                              )))
        }},
        Err(errors) => {{
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::{resources}(views::form::new(new_{resource}, errors))
                              )))
        }}
    }}
}}

pub fn update(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref db = *pool.get().unwrap();
    let mut {resource} = {resources}.find(id).first::<{Resource}>(db).unwrap();
    {resource}.update(params);

    match {resource}.is_valid() {{
        Ok(_) => {{
            {resource}.save_changes::<{Resource}>(db).unwrap();
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
    let (route, _params, pool) = read_request(request);
    let id = route.find("id").unwrap_or("").parse::<i32>().unwrap();
    let ref db = *pool.get().unwrap();
    let mut {resource} = {resources}.find(id).first::<{Resource}>(db).unwrap();
    Ok(Response::with((status::Found,
                       Header(headers::Location(format!("/{resource}/{{}}", {resource}.id))),
                       Header(headers::Connection::close())
                      )))
}}

