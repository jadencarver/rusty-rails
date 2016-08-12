use controllers::prelude::*;
use models::{resource}::*;
use schema::{resources}::dsl::{resources};

mod views {{
    pub mod form;
    pub mod index;
    pub mod show;
}}

/// The {resources}'s index action for
///   get /{resources}.html
pub fn index(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let results = itry!({resources}.get_results(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::index::index(results))
                      )))
}}

/// The {resources}'s show action for
///   get /{resources}/:id{{.format}}
pub fn show(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id: i32 = itry!(route.find("id").unwrap_or("").parse(), (status::BadRequest));
    let ref db = *pool.get().unwrap();
    let {resource} = itry!({resources}.find(id).first(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::show::show({resource}))
                      )))
}}

/// The {Resources}'s new {resource} form for
///   get /{resources}/:id{{.format}}
pub fn new(request: &mut Request) -> IronResult<Response> {{
    let {resource} = {Resource}::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::form::new({resource}, None))
                      )))
}}

/// The {Resources}'s edit {resources} form for
///   get /{resources}/:id{{.format}}
pub fn edit(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id: i32 = itry!(route.find("id").unwrap_or("").parse(), (status::BadRequest));
    let ref db = *pool.get().unwrap();
    let {resource} = itry!({resources}.find(id).first(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::{resources}(views::form::edit({resource}, None))
                      )))
}}

/// The {resources}'s create action for
///   post /{resources}
pub fn create(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let mut new_{resource} = {Resource}::new();
    new_{resource}.update(params);

    match new_{resource}.is_valid() {{
        Ok(_) => {{
            let {resource}: {Resource} = itry!(diesel::insert(&new_{resource}).into({resources}).get_result(db));
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

/// The {resources}'s update action for
///   PATCH /{resources}/:id{{.format}}
pub fn update(request: &mut Request) -> IronResult<Response> {{
    let (route, params, pool) = read_request(request);
    let id: i32 = itry!(route.find("id").unwrap_or("").parse(), (status::BadRequest));
    let ref db = *pool.get().unwrap();
    let mut {resource}: {Resource} = itry!({resources}.find(id).first(db));
    {resource}.update(params);

    match {resource}.is_valid() {{
        Ok(_) => {{
            itry!({resource}.save_changes::<{Resource}>(db));
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

/// The {resources}'s delete action for
///   DELETE /{resources}/:id{{.format}}
pub fn delete(request: &mut Request) -> IronResult<Response> {{
    let (route, _params, pool) = read_request(request);
    let id: i32 = itry!(route.find("id").unwrap_or("").parse(), (status::BadRequest));
    let ref db = *pool.get().unwrap();
    itry!(diesel::delete({resources}.find(id)).execute(db));
    Ok(Response::with((status::Found,
                       Header(headers::Location(format!("/{resources}"))),
                       Header(headers::Connection::close())
                      )))
}}

