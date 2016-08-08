use maud::PreEscaped;
use models::{resource}::*;

pub fn new({resource}: New{Resource}, errors: Errors) -> PreEscaped<String> {{
    let mut html = String::new();

    html!(html, {{
        form id="new_{resource}" action="/{resources}" method="post" {{
            h2 "Creating {Resource}"
            ^(form({resource}, errors))
            div class="actions" {{
                input type="submit" value="Create {Resource}" /
            }}
        }}
    }}).unwrap();

    PreEscaped(html)
}}

pub fn edit({resource}: {Resource}, errors: Errors) -> PreEscaped<String> {{
    let mut html = String::new();

    html!(html, {{
        form action=^(format!("/{resources}/{{}}?_method=patch", {resource}.id)) method="post" {{
            h2 "Editing {Resource}"
            ^(form({resource}, errors))
            div class="actions" {{
                input type="submit" value="Update {Resource}" /
            }}
        }}
    }}).unwrap();

    PreEscaped(html)
}}

fn form<T: {Resource}Model>({resource}: T, errors: Errors) -> PreEscaped<String> {{
    let mut html = String::new();
    html!(html, {{
        @if errors.is_some() {{
            ul.{resource}__errors.errors {{
                @for (field, messages) in errors.unwrap() {{
                    @for message in messages {{
                        li ^(format!("{{}} {{}}", field, message))
                    }}
                }}
            }}
        }}
{fields}
    }}).unwrap();
    PreEscaped(html)
}}
