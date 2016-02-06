use router::Router;
use std::path::Path;
use staticfile::Static;
use controllers::*;

pub fn routes() -> Router {
    router!(
        get "/" => pages::index,
        get "/entries" => entries::index,
        get "/entries/:id" => entries::show,
        get "/*" => Static::new(Path::new("public"))
    )
}
