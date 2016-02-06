use maud::PreEscaped;
use models::entry::Entry;

pub fn index(entries: Vec<Entry>) -> String {
  let mut body = String::new();

  html!(body, {

    ul class="container" {
      li {
        #for entry in entries {
          article id=$(format!("entry_{}", entry.id)) {
            a href=$(format!("/entries/{}", entry.id)) $(entry.title)
            $(entry.body)
          }
        }
      }
    }

  });

  body

}
