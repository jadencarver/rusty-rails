use maud::PreEscaped;
use models::entry::*;
use helpers::view::*;

pub fn index(entries: Vec<Entry>) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        h1 "Entries"
        table id="enties__index" {
            thead {
                tr {
                    th "Entry"
                    th "Title"
                    th "Description"
                }
            }
            tbody {
                @for entry in entries.iter() {
                    tr {
                        td a href=^(format!("/entries/{}", entry.id())) ^(entry)
                        td ^(entry.title())
                        td ^(entry.description())
                    }
                }
            }
        }
        ul.entries__actions.actions {
            li a href="/entries/new" "New Entry"
        }
    }).unwrap();

    PreEscaped(body)
}
