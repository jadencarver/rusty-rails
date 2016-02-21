use maud::PreEscaped;

pub fn simple_format(text: String) -> PreEscaped<String> {
    let mut result = String::new();

    let iter = text.lines();
    for line in iter {
        result.push_str(&format!("<p>{}</p>", line)[..]);
    }

    PreEscaped(result)
}
