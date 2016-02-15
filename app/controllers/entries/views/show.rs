use models::entry::Entry;
use helpers::view::simple_format;
use maud::PreEscaped;

pub fn show(entry: Entry) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article {
            h2 $(entry.title)
            $(simple_format(entry.body))
            div class="actions" {
                a href=$(format!("/entries/{}/edit", entry.id)) "Edit"
            }
        }
    }).unwrap();

    PreEscaped(body)
}
