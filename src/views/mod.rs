use maud::{PreEscaped};

pub fn layout(body: String) -> String {
	let mut page = String::new();

	html!(page, {
		html {
			head {
				title "Rusty Rails"
				link rel="stylesheet" href="/style.css" /
			}
			body {
				$PreEscaped(body)
			}
		}
	});

	page
}