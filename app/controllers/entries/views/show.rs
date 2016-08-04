use models::entry::*;
use maud::PreEscaped;

pub fn show(entry: Entry) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article.entry id=^(format!("entry_{}", entry.id)) {


                div.entry__title {
                    span "title:"
                    span ^(entry.title())
                }

                div.entry__description {
                    span "description:"
                    span ^(entry.description())
                }


            ul.entry__actions.actions {
                li a href=^(format!("/entries/{}/edit", entry.id)) "Edit Entry"
                li a href="/entries" "View All"
            }
        }
    }).unwrap();

    PreEscaped(body)
}
