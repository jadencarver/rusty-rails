use iron::prelude::*;
use std::error::Error;
use maud::PreEscaped;

pub fn default(error: IronError) -> PreEscaped<String> {
    let mut body = String::new();
    html!(body, {
        section id="error" {
            h1 ^(format!("{}", error.response.status.unwrap()))
                p ^(error.description())
        }
    }).unwrap();

    PreEscaped(body)
}
