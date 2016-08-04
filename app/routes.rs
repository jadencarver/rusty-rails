use router::Router;
use controllers::*;

pub fn routes() -> Router {
    router!(
        get    "/" => pages::index,
    )
}
