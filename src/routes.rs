extern crate iron;
extern crate handlebars_iron as handlebars;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate logger;
extern crate params;

#[macro_use]
extern crate router;
use iron::prelude::{Iron, Chain};

// Controllers
mod content;

fn main() {

  // Routes
  let router = router!(
    get "/"       => content::index,
    get "/about"  => content::about
  );

  // Mounts
  let mut mount = mount::Mount::new();
  mount
    .mount("/", router)
    .mount("/assets/", staticfile::Static::new(std::path::Path::new("assets/")));

  // Stack
  let (logger_before, logger_after) = logger::Logger::new(None);
  let hbs = handlebars::HandlebarsEngine::new("views/", ".hbs");
  let mut chain = Chain::new(mount);
  chain.link_before(logger_before);
  chain.link_after(logger_after);
  chain.link_after(hbs);
  Iron::new(chain).http("localhost:3000").unwrap();
}
