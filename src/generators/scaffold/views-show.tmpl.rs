use models::{resource}::*;
use maud::PreEscaped;

pub fn show({resource}: {Resource}) -> PreEscaped<String> {{
    let mut body = String::new();

    html!(body, {{
        article.{resource} id=^(format!("{resource}_{{}}", {resource}.id)) {{

{fields}

            ul.{resource}__actions.actions {{
                li a href=^(format!("/{resources}/{{}}/edit", {resource}.id)) "Edit {Resource}"
                li a href="/{resources}" "View All"
            }}
        }}
    }}).unwrap();

    PreEscaped(body)
}}
