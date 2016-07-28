use router::Router;
use staticfile::Static;
use std::path::Path;
use std::time::Duration;
use controllers::*;

pub fn routes() -> Router {
    let mut routes = router!(
        get "/" => pages::index,
        //get "/entries" => entries::index,
        //get "/entries/new" => entries::new,
        //get "/entries/:id" => entries::show,
        //get "/entries/:id/edit" => entries::edit,
        //post "/entries" => entries::create,
        //patch "/entries/:id" => entries::update,
        // post "/entries/:id" => entries::update,
        //delete "/entries/:id" => entries::delete,
    );
    routes.get("/assets/app/assets/*path", Static::new(Path::new("app/assets/")));
    routes.get("/assets/vendor/assets/*path", Static::new(Path::new("vendor/assets/")));
    routes.get("/*path", Static::new(Path::new("public/"))); //.cache(Duration::from_secs(30*24*60*60)));
    routes
}
