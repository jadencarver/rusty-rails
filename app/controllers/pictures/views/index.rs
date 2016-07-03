use maud::PreEscaped;
use models::picture::*;
use helpers::view::*;

pub fn index(pictures: Vec<Picture>) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        ul id="picture_index" {
            @for picture in pictures.iter() {
                li ^(picture)
            }
        }
    }).unwrap();

    PreEscaped(body)
}
