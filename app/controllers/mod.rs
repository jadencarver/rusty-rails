pub use iron::prelude::*;
pub use iron::modifiers::*;
pub use iron::status;
pub use layouts;
pub use formats;

pub use diesel;
pub use diesel::prelude::*;

use persistent;
use params;
use router;

//pub mod entries;
pub mod pages;

// pub fn params(request: &mut Request) -> (router::Params, params::Map, ::DBPoolRef) {
//     (
//         request.extensions.get::<router::Router>().unwrap().clone(),
//         request.get::<params::Params>().unwrap().clone(),
//         request.extensions.get::<persistent::Read<::DB>>().unwrap().clone()
//     )
// }
