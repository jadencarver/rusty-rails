use router::Router;
use std::path::Path;
use staticfile::Static;
use controllers::*;

pub fn routes() -> Router {
    router!(
        get "/" => pages::index,
        get "/entries" => entries::index,
        get "/entries/new" => entries::new,
        get "/entries/:id" => entries::show,
        get "/entries/:id/edit" => entries::edit,
        patch "/entries/:id" => entries::update,
        post "/entries/:id" => entries::update,
        //delete "/entries/:id" => entries::delete,
        get "/*" => Static::new(Path::new("public"))
    )
}
