use controllers::prelude::*;

mod views {
    pub mod index;
}

pub fn index(_: &mut Request) -> IronResult<Response> {
  Ok(Response::with((
    status::Ok,
    Header(formats::html()),
    layouts::pages(views::index::index())
  )))
}
