use models::entry::Entry;
use helpers::view::simple_format;
use maud::PreEscaped;

pub fn show(entry: Entry) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article {
            section {}
            section {
                h2 ^(entry.title)
            }
            div ^(simple_format(entry.body))
            section {
                ul class="actions" {
                    li a href=^(format!("/entries/{}/edit", entry.id)) "Edit Entry"
                    li a href="/entries" "View All"
                }
            }
        }
    }).unwrap();

    PreEscaped(body)
}
