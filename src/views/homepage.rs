pub fn homepage() -> String {
	let mut body = String::new();

	html!(body, {
		article {
			section {
				h1 "Rusty Rails"
				p "You are riding Rusty Rails!"
			}
			section {}
			section {
				h2 "Getting started"
				ol {
					li { "Use " code { "cargo run scaffold" } " to create a RESTful resource." }
					li { "Create and update the database using " code { "cargo run migrate" } "." }
					li { "Access the " a href="/_rusty/analytics" { "built in analytics dashboard" } "" }
				}
			}
		}
	});

	body
}