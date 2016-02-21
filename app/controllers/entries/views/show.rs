use models::entry::Entry;
use helpers::view::simple_format;
use maud::PreEscaped;

pub fn show(entry: Entry) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article {
            section {
                h2 ^(entry.title)
            }
            ^(simple_format(entry.body))
            section {
                div class="actions" {
                    a href=^(format!("/entries/{}/edit", entry.id)) "Edit Entry"
                }
            }
        }
    }).unwrap();

    PreEscaped(body)
}
