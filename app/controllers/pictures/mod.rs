use controllers::prelude::*;
use models::picture::*;
use schema::pictures::dsl::pictures;
mod views;

pub fn index(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let results = try!(pictures.get_results::<Picture>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::pictures(views::index::index(results))
                      )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let picture = try!(pictures.find(id).first::<Picture>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::pictures(views::show::show(picture))
                      )))
}

pub fn new(request: &mut Request) -> IronResult<Response> {
    let picture = Picture::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::pictures(views::form::new(picture, None))
                      )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let picture = try!(pictures.find(id).first::<Picture>(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::pictures(views::form::edit(picture, None))
                      )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let ref db = *pool.get().unwrap();
    let mut new_picture = Picture::new();
    new_picture.update(params);

    match new_picture.is_valid() {
        Ok(_) => {
            let picture: Picture = try!(diesel::insert(&new_picture).into(pictures).get_result(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/picture/{}", picture.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors) => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::pictures(views::form::new(new_picture, errors))
                              )))
        }
    }
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let mut picture = try!(pictures.find(id).first::<Picture>(db));
    picture.update(params);

    match picture.is_valid() {
        Ok(_) => {
            try!(pictures.save_changes::<Picture>(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/pictures/{}", picture.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors)  => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::pictures(views::form::edit(picture, errors))
                              )))
        }
    }
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
    let (route, _params, pool) = read_request(request);
    let id = try!(route.find("id").unwrap_or("").parse::<i32>());
    let ref db = *pool.get().unwrap();
    let mut picture = try!(pictures.find(id).first::<Picture>(db));
    Ok(Response::with((status::Found,
                       Header(headers::Location(format!("/picture/{}", picture.id))),
                       Header(headers::Connection::close())
                      )))
}

