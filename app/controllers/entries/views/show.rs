pub fn show() -> String {
    let mut body = String::new();

    html!(body, {
        article {
            h1 "Entry Show Page"
        }
    });

    body
}
