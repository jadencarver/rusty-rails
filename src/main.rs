extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;

#[macro_use]
extern crate router;

use std::path::Path;
use iron::prelude::*;
use hbs::HandlebarsEngine;
use staticfile::Static;
use mount::Mount;

// Controllers
mod content;

fn main() {

  // Routes
  let router = router!(
    get "/"       => content::index,
    get "/about"  => content::about
  );

  // Mounts
  let mut mount = Mount::new();
  mount
    .mount("/", router)
    .mount("/assets/", Static::new(Path::new("assets/")));
  
  // Stack
  let hbs = HandlebarsEngine::new("views/", ".hbs");
  let mut main = Chain::new(mount);
  main.link_after(hbs);
  Iron::new(main).http("localhost:3000").unwrap();
}