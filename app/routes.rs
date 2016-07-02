use controllers::*;
use router::Router;
use staticfile::Static;
use std::path::Path;
use std::time::Duration;

pub fn routes() -> Router {
    router!(
        get "/" => pages::index,
        get "/resume" => pages::resume,
        get "/portfolio" => pages::portfolio,
        get "/entries" => entries::index,
        get "/entries/new" => entries::new,
        get "/entries/:id" => entries::show,
        get "/entries/:id/edit" => entries::edit,
        post "/entries" => entries::create,
        patch "/entries/:id" => entries::update,
         post "/entries/:id" => entries::update,
        delete "/entries/:id" => entries::delete,
        get "/*" => Static::new(Path::new("public")).cache(Duration::from_secs(30*24*60*60))
    )
}
