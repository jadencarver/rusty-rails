use maud::PreEscaped;

pub fn application(body_class: &str, body: PreEscaped<String>) -> String {
  let mut page = String::new();

  html!(page, {
    html {
      head {
        title "Rusty Rails"
        link rel="stylesheet" href="/assets/stylesheets.css" /
        script src="/assets/javascripts.js" {}
      }
      body class=^(body_class) ^(body)
    }
  }).unwrap();

  page
}

pub fn entries(body: PreEscaped<String>) -> String {
    application("entries", body)
}

pub fn pages(body: PreEscaped<String>) -> String {
    application("pages", body)
}
