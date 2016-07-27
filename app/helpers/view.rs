use maud::PreEscaped;
use std::fmt::Display;

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
    if text.len() > length {
        (&text[0..length]).to_string() + "..."
    } else {
        (&text[0..]).to_string()
    }
}

pub fn pluralize<T: Display>(text: T) -> String {
	let orig_string = format!("{}", text);
	let string = orig_string.trim();
	if string == "" { return String::new() }
	let string_lowercase = string.to_lowercase();
	let string_last_char = string_lowercase.chars().last().unwrap();
	let string_second_to_last_char = string_lowercase.chars().rev().nth(1).unwrap_or(' ');
	match string_last_char {
		'y' => match string_second_to_last_char {
			'a' | 'e' | 'i' | 'o' | 'u' => format!("{}s", string),
			_ => format!("{}ies", &string[0..string.len()-1])
		},
		'h' if string_second_to_last_char == 'c' || string_second_to_last_char == 's' => format!("{}es", string),
		'x' | 's' | 'z' | 'o' => format!("{}es", string),
		_ => match string_lowercase.as_ref() {
			"goose" => format!("{:.1}eese", string),
			"knife" | "loaf" => format!("{:.3}ves", string),
			"leaf" => format!("{:.3}ves", string),
			"deer" => format!("{}", string),
			_ => format!("{}s", string)
		}
	}
}

#[test]
fn test_pluralize() {
}
