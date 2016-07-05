pub mod prelude {
    use persistent;
    pub use params;
    pub use router;
    pub use diesel;
    pub use formats;
    pub use layouts;
    pub use iron::status;
    pub use iron::headers;
    pub use iron::prelude::*;
    pub use iron::modifiers::*;
    pub use diesel::prelude::*;

    pub fn read_request(request: &mut Request) -> (router::Params, params::Map, ::DBPoolArc) {
        (
            request.extensions.get::<router::Router>().unwrap().clone(),
            request.get::<params::Params>().unwrap().clone(),
            request.get::<persistent::Read<::DB>>().unwrap().clone()
        )
    }
}

pub mod pages;
