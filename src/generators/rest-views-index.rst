use maud::PreEscaped;
use models::{resource}::*;
use helpers::view::*;

pub fn index({resources}: Vec<{Resource}>) -> PreEscaped<String> {{
    let mut body = String::new();

    html!(body, {{
        ul id="{resource}_index" {{
            @for {resource} in {resources}.iter() {{
                li ^({resource})
            }}
        }}
    }}).unwrap();

    PreEscaped(body)
}}
