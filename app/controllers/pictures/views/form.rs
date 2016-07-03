use maud::PreEscaped;
use models::picture::*;

pub fn new(picture: NewPicture, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();

    html!(html, {
        form id="new_picture" action="/pictures" method="POST" {
            h2 "Creating Picture"
            ^(form(picture, errors))
            div class="actions" {
                input type="submit" value="Create Picture" /
            }
        }
    }).unwrap();

    PreEscaped(html)
}

pub fn edit(picture: Picture, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();

    html!(html, {
        form action=^(format!("/pictures/{}", picture.id)) method="PATCH" {
            h2 "Editing Picture"
            ^(form(picture, errors))
            div class="actions" {
                input type="submit" value="Update Picture" /
            }
        }
    }).unwrap();

    PreEscaped(html)
}

fn form<T: PictureModel>(picture: T, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {
        @if errors.is_some() {
            ul.picture__errors.errors {
                @for (field, messages) in errors.unwrap() {
                    @for message in messages {
                        li ^(format!("{} {}", field, message))
                    }
                }
            }
        }

        div.picture__title.field {
            label for="picture__title" "title"
            input  id="picture__title" type="String" name="picture[title]" ^(picture.title())
        }

    }).unwrap();
    PreEscaped(html)
}
