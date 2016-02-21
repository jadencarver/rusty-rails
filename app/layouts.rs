use maud::PreEscaped;

pub fn application(body: PreEscaped<String>) -> String {
  let mut page = String::new();

  html!(page, {
    html {
      head {
        title "Rusty Rails"
        link rel="stylesheet" href="/assets/stylesheets.css" /
        script src="/assets/javascripts.js" {}
      }
      body ^(body)
    }
  }).unwrap();

  page
}
