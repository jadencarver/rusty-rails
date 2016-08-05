use maud::PreEscaped;
use models::entry::*;

pub fn new(entry: NewEntry, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();

    html!(html, {
        form id="new_entry" action="/entries" method="POST" {
            h2 "New Entry"
            ^(form(entry, errors))
            ul class="actions" {
                li input type="submit" value="Create Entry" /
                li a href="/entries" "Cancel"
            }
        }
    }).unwrap();

    PreEscaped(html)
}

pub fn edit(entry: Entry, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();

    html!(html, {
        form action=^(format!("/entries/{}?_method=patch", entry.id)) method="POST" {
            h2 "Edit Entry"
            ^(form(entry, errors))
            ul class="actions" {
                li input type="submit" value="Update Entry" /
                li a href="/entries" "Cancel"
            }
        }
    }).unwrap();

    PreEscaped(html)
}

fn form<T: EntryModel>(entry: T, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {
        @if errors.is_some() {
            ul.entry__errors.errors {
                @for (field, messages) in errors.unwrap() {
                    @for message in messages {
                        li ^(format!("{} {}", field, message))
                    }
                }
            }
        }

        div.entry__title.field {
            label for="entry__title" "title"
            input  id="entry__title" type="title" name="entry[title]" value=^(entry.title()) /
        }

        div.entry__description.field {
            label for="entry__description" "description"
            input  id="entry__description" type="description" name="entry[description]" value=^(entry.description()) /
        }

    }).unwrap();
    PreEscaped(html)
}
