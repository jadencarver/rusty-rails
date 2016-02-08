use models::entry::Entry;
use helpers::view::simple_format;

pub fn show(entry: Entry) -> String {
    let mut body = String::new();

    html!(body, {
        article {
            h2 $(entry.title)
            $(simple_format(entry.body))
        }
    });

    body
}
