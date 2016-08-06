use router::Router;
use controllers::*;

pub fn routes() -> Router {
    router!(
        get    "/" => pages::index,
        get    "/entries"          => entries::index,
        get    "/entries/new"      => entries::new,
        get    "/entries/:id"      => entries::show,
        get    "/entries/:id/edit" => entries::edit,
        post   "/entries"          => entries::create,
        patch  "/entries/:id"      => entries::update,
        delete "/entries/:id"      => entries::delete,
        get    "/users"          => users::index,
        get    "/users/new"      => users::new,
        get    "/users/:id"      => users::show,
        get    "/users/:id/edit" => users::edit,
        post   "/users"          => users::create,
        patch  "/users/:id"      => users::update,
        delete "/users/:id"      => users::delete,
    )
}
