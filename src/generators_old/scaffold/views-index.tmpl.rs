use maud::PreEscaped;
use models::{resource}::*;
use helpers::view::*;

pub fn index({resources}: Vec<{Resource}>) -> PreEscaped<String> {{
    let mut body = String::new();

    html!(body, {{
        h1 "{Resource}s"
        table id="{resources}__index" {{
            thead {{
                tr {{
                    th "{Resource}"{headers}
                }}
            }}
            tbody {{
                @for {resource} in {resources}.iter() {{
                    tr {{
                        td a href=^(format!("/{resources}/{{}}", {resource}.id())) ^({resource}){rows}
                    }}
                }}
            }}
        }}
        ul.{resources}__actions.actions {{
            li a href="/{resources}/new" "New {Resource}"
        }}
    }}).unwrap();

    PreEscaped(body)
}}
