use maud::PreEscaped;

pub fn simple_format(text: String) -> PreEscaped<String> {
    let mut result = String::new();

    let paragraphs = text.split("\r\n\r\n");
    html!(result, {
        @for paragraph in paragraphs {
            p {
                @for line in paragraph.lines() {
                    ^(line)
                    br /
                }
            }
        }
    }).unwrap();

    PreEscaped(result)
}

pub fn truncate(text: String, length: usize) -> String {
    let t = &text[0..length];
    t.to_string() + "..."
}
