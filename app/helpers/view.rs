use maud::PreEscaped;

pub fn simple_format(text: String) -> PreEscaped<String> {
    PreEscaped(format!("<p>{}</p>", text))
}
