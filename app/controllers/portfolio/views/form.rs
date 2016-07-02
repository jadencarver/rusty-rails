use maud::PreEscaped;
use models::portfolio::{NewPortfolio, Portfolio, Errors};

pub fn new(portfolio: NewPortfolio, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form id=^(format!("portfolios_{}", portfolio.id)) action="/portfolios" method="POST" {
            h2 "Creating Portfolio"
            ^(form(portfolio.to_generic(), errors))
            div class="actions" {
                input type="submit" value="Create Portfolio" /
            }
        }

    }).unwrap();
    PreEscaped(html)
}

pub fn edit(portfolio: Portfolio, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {

        form action=^(format!("/portfolios/{}", portfolio.id)) method="POST" {
            h2 "Editing Portfolio"
            ^(form(portfolio, errors))
            div class="actions" {
                input type="submit" value="Update Portfolio" /
            }
        }

    }).unwrap();
    PreEscaped(html)
}

fn form(portfolio: Portfolio, errors: Errors) -> PreEscaped<String> {
    let mut html = String::new();
    html!(html, {
        @if errors.is_some() {
            ul {
                @for (field, messages) in errors.unwrap() {
                    @for message in messages {
                        li ^(format!("{} {}", field, message))
                    }
                }
            }
        }

        div class="field" {
            label for="portfolio_title" "Title"
            input id="portfolio_title" type="text" name="portfolio[title]" value=^(portfolio.title) /
        }

        div class="field" {
            label for="portfolio_body" "Body"
            textarea id="portfolio_body" type="text" name="portfolio[body]" ^(portfolio.body)
        }

    }).unwrap();
    PreEscaped(html)
}
