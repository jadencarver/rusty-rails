use maud::PreEscaped;
use models::entry::Entry;
use helpers::view::*;

pub fn index(entries: Vec<Entry>) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {

        ul class="container" {
            li {
                @for entry in entries {
                    article id=^(format!("entry_{:?}", entry.id)) {
                        h4 a href=^(format!("/entries/{:?}", entry.id)) ^(entry.title)
                            ^(simple_format(truncate(entry.body, 175)))
                    }
                }
            }
        }

    }).unwrap();

    PreEscaped(body)
}
