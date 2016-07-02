use maud::PreEscaped;

pub fn index(portfolios: Vec<Portfolio>) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        ul id="portfolio_index" {
            @for portfolio in portfolios {
                li ^(portfolio)
            }
        }
    }).unwrap();

    PreEscaped(body)
}
