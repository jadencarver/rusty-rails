use models::entry::*;
use maud::PreEscaped;

pub fn show(entry: Entry) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article.entry id=^(format!("entry_{}", entry.id)) {
            h1 ^(entry)

            div.entry__title {
                span ^(format!("{}: ", "title"))
                span ^(entry.title())
            }

            div.entry__description {
                span ^(format!("{}: ", "description"))
                span ^(entry.description())
            }

            ul.entry__actions.actions {
                li a href="/entries" "View All"
                li a href=^(format!("/entries/{}/edit", entry.id)) "Edit Entry"
                li form method="post" action=^(format!("/entries/{}?_method=delete", entry.id)) button type="submit" "Delete Entry"
            }
        }
    }).unwrap();

    PreEscaped(body)
}
