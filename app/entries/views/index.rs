use maud::PreEscaped;
use entries::entry::Entry;

// pub fn simple_format(text: String) -> PreEscaped<String> {
//   let paragraphs = text.split("\n\n").collect().unwrap();
//   html!(text, {
//     #for paragraph in paragraphs {
//       p $(paragraph)
//     }
//   });
//   PreEscaped(text)
// }

pub fn index(entries: Vec<Entry>) -> String {
  let mut body = String::new();

  html!(body, {

    ol class="container" {
      li {
        #for entry in entries {
          article id="entry_$(entry.id)" {
            h3 $(entry.title)
            // $(simple_format(entry.title))
          }
        }
      }
    }

  });

  body

}