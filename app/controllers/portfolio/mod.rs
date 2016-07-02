use controllers::*;
use models::portfolio::Portfolio;
use schema::portfolios::dsl::portfolios;
mod views;

pub fn index(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
    let results = try!(portfolios.get_results::<Portfolio>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::portfolio(views::index::index(results))
                      )))
}

pub fn show(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = params(request);
    let id = try!(try!(route.find("id")).parse::<i32>());
    let ref db = try!(*pool.get());
    let portfolio = try!(portfolio.find(id).first::<Portfolio>(db));

    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::portfolio(views::show::show())
                      )))
}

pub fn new(request: &mut Request) -> IronResult<Response> {
    let portfolio = Portfolio::new();
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::portfolio(views::form::new(portfolio))
                      )))
}

pub fn edit(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = params(request);
    let id = try!(try!(route.find("id")).parse::<i32>());
    let ref db = try!(*pool.get());
    let portfolio = try!(portfolio.find(id).first::<Portfolio>(db));
    Ok(Response::with((status::Ok,
                       Header(formats::html()),
                       layouts::portfolio(views::form::edit())
                      )))
}

pub fn create(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
    let mut new_portfolio = Portfolio::new();
    new_portfolio.update(params);

    match new_portfolio.is_valid() {
        Ok(_) => {
            let portfolio: Portfolio = try!(diesel::insert(&new_portfolio).into(portfolios).get_result(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/portfolio/{}", new_portfolio.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors) => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::portfolio(views::form::new(portfolio, errors))
                              )))
        }
    }
}

pub fn update(request: &mut Request) -> IronResult<Response> {
    let (route, params, pool) = params(request);
    let ref db = try!(*pool.get());
    let id = try!(try!(route.find("id")).parse::<i32>());
    let mut portfolio = try!(portfolios.find(id).first::<Portfolio>(db));
    portfolio.update(params);

    match portfolio.is_valid() {
        Ok(_) => {
            try!(portfolio.save_changes::<Portfolio>(db));
            Ok(Response::with((status::Found,
                               Header(headers::Location(format!("/portfolios/{}", portfolio.id))),
                               Header(headers::Connection::close())
                              )))
        },
        Err(errors)  => {
            Ok(Response::with((status::NotAcceptable,
                               Header(formats::html()),
                               layouts::portfolios(views::form::edit(portfolio, errors))
                              )))
        }
    }
}

pub fn delete(request: &mut Request) -> IronResult<Response> {
     Ok(Response::with((status::Found,
                        Header(headers::Location(format!("/portfolio/{}", new_portfolio.id))),
                        Header(headers::Connection::close())
                       )))
}

