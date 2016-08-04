use maud::PreEscaped;
use models::entry::*;
use helpers::view::*;

pub fn index(entries: Vec<Entry>) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        ul id="entry_index" {
            @for entry in entries.iter() {
                li a href=^(format!("/entries/{}", entry.id())) ^(entry)
            }
        }
        a href="/entries/new" { "New Entry" }
    }).unwrap();

    PreEscaped(body)
}
