use controllers::*;
mod views;

pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
                status::Ok,
                Header(formats::html()),
                layouts::pages(views::index::index())
                )))
}

pub fn resume(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
                status::Ok,
                Header(formats::html()),
                layouts::pages(views::resume::resume())
                )))
}

pub fn portfolio(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
                status::Ok,
                Header(formats::html()),
                layouts::pages(views::portfolio::portfolio())
                )))
}
