//pub mod entries;
pub mod pages;

use iron::prelude::*;
use persistent;
use params;
use router;

// pub fn params(request: &mut Request) -> (router::Params, params::Map, ::DBPoolRef) {
//     (
//         request.extensions.get::<router::Router>().unwrap().clone(),
//         request.get::<params::Params>().unwrap().clone(),
//         request.extensions.get::<persistent::Read<::DB>>().unwrap().clone()
//     )
// }
