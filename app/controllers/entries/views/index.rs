use maud::PreEscaped;
use models::entry::Entry;
use helpers::view::*;

pub fn index(entries: Vec<Entry>, page: i64, num_pages: i64) -> PreEscaped<String> {
    let mut body = String::new();
    let mut low = page - 3;
    if low < 0 { low = 0 }
    let mut high = page + 3;
    if high > num_pages { high = num_pages + 1 }

    html!(body, {

        section {
            ul class="actions" style="text-align: center;" {
                @if page > 0 {
                    li a href=^(format!("/entries?p={}", page-1)) "Previous"
                }
                @for p in low..page {
                    li a href=^(format!("/entries?p={}", p)) ^(p+1)
                }
                li ^(page+1)
                @for p in page+1..high {
                    li a href=^(format!("/entries?p={}", p)) ^(p+1)
                }
                @if page < num_pages {
                    li a href=^(format!("/entries?p={}", page+1)) "Next Page"
                }
            }
        }

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

        section {
            ul class="actions" {
                li a href="/entries/new" "New Entry"
            }
        }

    }).unwrap();

    PreEscaped(body)
}
