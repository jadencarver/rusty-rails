use maud::PreEscaped;

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
