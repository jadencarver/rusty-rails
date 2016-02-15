use maud::PreEscaped;
use models::entry::Entry;

pub fn new(entry: Entry) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form action="/entries" method="POST" {
            h2 "Creating Entry"
            $(form(entry))
            div class="actions" {
                input type="submit" value="Create Entry" /
            }
        }

    }).unwrap();
    PreEscaped(html)
}

pub fn edit(entry: Entry) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form action=$(format!("/entries/{}", entry.id)) method="POST" {
            h2 "Editing Entry"
            $(form(entry))
            div class="actions" {
                input type="submit" value="Update Entry" /
            }
        }

    }).unwrap();
    PreEscaped(html)
}

fn form(entry: Entry) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        div class="field" {
            label for="entry_title" "Title"
            input id="entry_title" type="text" name="entry[title]" value=$(entry.title) /
        }

        div class="field" {
            label for="entry_body" "Body"
            input id="entry_body" type="text" name="entry[body]" value=$(entry.body) /
        }

    }).unwrap();
    PreEscaped(html)
}
