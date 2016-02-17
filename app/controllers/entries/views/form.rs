use maud::PreEscaped;
use models::entry::{Entry, Errors};

pub fn new(entry: Entry, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form action="/entries" method="POST" {
            h2 "Creating Entry"
                $(form(entry, errors))
                div class="actions" {
                    input type="submit" value="Create Entry" /
                }
        }

    }).unwrap();
    PreEscaped(html)
}

pub fn edit(entry: Entry, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form action=$(format!("/entries/{}", entry.id)) method="POST" {
            h2 "Editing Entry"
            $(form(entry, errors))
                div class="actions" {
                    input type="submit" value="Update Entry" /
                }
        }

    }).unwrap();
    PreEscaped(html)
}

// -- private --
//
fn form(entry: Entry, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {
        #if errors.is_some() {
            ul {
                #for (field, messages) in errors.unwrap() {
                    #for message in messages {
                        li $(format!("{} {}", field, message))
                    }
                }
            }
        }

        div class="field" {
            label for="entry_title" "Title"
                input id="entry_title" type="text" name="entry[title]" value=$(entry.title) /
        }

        div class="field" {
            label for="entry_body" "Body"
                textarea id="entry_body" type="text" name="entry[body]" $(entry.body)
        }

    }).unwrap();
    PreEscaped(html)
}
