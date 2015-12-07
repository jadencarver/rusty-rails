extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
extern crate staticfile;
extern crate mount;
extern crate mysql;
extern crate logger;

#[macro_use]
extern crate router;

use std::path::Path;
use iron::prelude::*;
use hbs::HandlebarsEngine;
use staticfile::Static;
use mount::Mount;

use std::default::Default;
use logger::Logger;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_row;

// Controllers
mod content;

fn main() {
  let opts = MyOpts {
    user: Some("root".to_string()),
    pass: None,
    db_name: Some("rusty_rails_dev".to_string()),
    ..Default::default()
  };
  let pool = MyPool::new(opts).unwrap();
  pool.prep_exec(r"DROP TABLE IF EXISTS requests", ()).unwrap();
  pool.prep_exec(r"CREATE TABLE requests (
                     customer_id int not null
                  )", ()).unwrap();

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
  let (logger_before, logger_after) = Logger::new(None);
  let hbs = HandlebarsEngine::new("views/", ".hbs");
  let mut chain = Chain::new(mount);
  chain.link_before(logger_before);
  chain.link_after(logger_after);
  chain.link_after(hbs);
  Iron::new(chain).http("localhost:3000").unwrap();
}