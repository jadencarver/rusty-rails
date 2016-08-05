#![feature(custom_derive, custom_attribute, plugin, stmt_expr_attributes)]
#![plugin(maud_macros, diesel_codegen, dotenv_macros)]

/// # Rusty Rails uses the Iron Framework
///
///
#[macro_use] extern crate iron;
#[macro_use] extern crate router;
pub extern crate params;
extern crate logger;
extern crate staticfile;

/// ## Maud for Templating is Schweet
///
/// html!(buffer, {
///   html {
///     body {
///       h1 "Hello World!"
///       p class="leading" style="font-size: 1.1rem;" {
///         @first_paragraph
///       }
///       p @(second_paragraph)
///     }
///   }
/// }
extern crate maud;

/// ## Diesel provides an ORM (PostgreSQL/SQLLite)
/// 
/// If you'd rather use plain SQL, it can easily be swapped out for the r2d2-postgres and postgres
/// cargo packages.
///
#[macro_use] pub extern crate diesel;
extern crate ansi_term;
extern crate dotenv;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_diesel;

use iron::prelude::*;
use iron::modifiers::*;
use ansi_term::Colour::*;
use dotenv::dotenv;
use iron::{AfterMiddleware, BeforeMiddleware};
use iron::status;
use logger::Logger;
use std::env;
use std::path::Path;
use staticfile::Static;
use std::time::Duration;

/// You can change also change the type of database used by the connection pool.
pub type DBType = diesel::pg::PgConnection;
pub type DBManager = r2d2_diesel::ConnectionManager<DBType>;
pub type DBPool = r2d2::Pool<DBManager>;
pub type DBPoolArc = std::sync::Arc<DBPool>;

pub struct DB;
impl iron::typemap::Key for DB {
    type Value = DBPool;
}

/// ### Routing and MVC
/// The design of Rusty Rails is inspired by (obviously) Ruby on Rails, so it should be instantly
/// familiar to you.  One big difference is that I prefer my views folders to sit right next to
/// their controllers.  It may be helpful for readers to familiarize themselves with the Rust
/// pattern for modularizing code.  Basically, you can create files with their module name,
/// (like `controllers/entries.rs` maps to `controllers::entries`) or you can break it down even
/// further by creating a folder containing at least one file: `controllers/entries/mod.rs`.
///
/// Iron's router is really easy to use, and you can manage them inside `app/routes.rs`.  The final
/// route serves static assets.  If you don't want to (like in a production environment, behind
/// nginx, then feel free to remove it, but it is harmless either way.
/// 
/// ```
/// router!(
///    // root
///    get "/" => pages::index,
///
///    // RESTful
///    get "/entries" => entries::index,
///    get "/entries/new" => entries::new,
///    get "/entries/:id" => entries::show,
///    get "/entries/:id/edit" => entries::edit,
///    patch "/entries/:id" => entries::update,
///    delete "/entries/:id" => entries::delete,
///
///    // Static
///    get "/*" => Static::new(Path::new("public"))
/// )
/// ```
mod routes;

/// ### Controllers
/// Controller functions (marked `pub`) are expected to respond to each request with an
/// appropriate status code, content-type and body.
///
pub mod controllers;

/// ### Helpers
/// Various helpers are available, and custom helpers can be defined in app/helpers.
pub mod helpers;
/// Define your layout in `layouts.rs` multiple layouts can be defined, or broken up into a
/// module inside of the `layouts/` folder.
pub mod layouts;

pub mod formats;
pub mod schema;
pub mod models;
mod errors;

/// ### Error Handling
/// This middleware captures errors and displays an error page.  Customize it in `app/errors.rs`.

struct ErrorHandler;
impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, error: IronError) -> IronResult<Response> {
        Ok(Response::with((status::Ok,
                           Header(formats::html()),
                           layouts::application("errors", errors::default(error))
                          )))
    }
}

struct MethodOverride;
impl BeforeMiddleware for MethodOverride {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        use iron::method::Method;
        use params::Value;
        let params = request.get::<params::Params>().unwrap();
        if request.method == Method::Post {
            if let Some(value) = params.find(&["_method"]) {
                request.method = match value {
                    &Value::String(ref value) if value == "delete" => Method::Delete,
                    &Value::String(ref value) if value == "patch" => Method::Patch,
                    _ => Method::Post
                }
            }
        };
        Ok(())
    }
}


// Here we go!!
// ------------------
fn main() {
    
    // Provide secrets and environment variables using `.env`
    dotenv().ok();
    let hostname = env::var("HOSTNAME")
        .expect("HOSTNAME must be set");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Iron acts as the router and middleware chain.
    let mut routes = routes::routes();
    #[cfg(debug_assertions)]
    {
        routes.get("/assets/app/assets/*path", Static::new(Path::new("app/assets/")));
        routes.get("/assets/vendor/assets/*path", Static::new(Path::new("vendor/assets/")));
        routes.get("/*path", Static::new(Path::new("public/")));
    }
    #[cfg(not(debug_assertions))]
    routes.get("/*path", Static::new(Path::new("public/")).cache(Duration::from_secs(30*24*60*60)));
    
    let mut chain = Chain::new(routes);

    // Iron and r2d2 provide persistent database connection pooling for all requests.
    let manager = r2d2_diesel::ConnectionManager::<DBType>::new(database_url);
    let pool = r2d2::Pool::new(r2d2::Config::default(), manager)
        .expect("Database connection failed.");
    chain.link(persistent::Read::<DB>::both(pool));

    let (logger_before, logger_after) = Logger::new(None);
    chain
        .link_before(MethodOverride)
        .link_before(logger_before)
        .link_after(logger_after)
        .link_after(ErrorHandler);

    // Fire-up them engines!
    match Iron::new(chain).http(&hostname[..]) {
        Ok(_) => println!("Started on {}", Green.bold().paint(hostname)),
        Err(error) => println!("Unable to start: {}", error)
    }
}

