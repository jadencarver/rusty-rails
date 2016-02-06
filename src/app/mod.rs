pub mod entries;
pub mod pages;

use maud::PreEscaped;

pub fn layout(body: String) -> String {
  let mut page = String::new();

  html!(page, {
    html {
      head {
        title "Rusty Rails"
        link rel="stylesheet" href="/assets/style.css" /
      }
      body {
        $PreEscaped(body)
      }
    }
  }).unwrap();

  page
}