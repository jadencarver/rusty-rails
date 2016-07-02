use models::portfolio::Portfolio;
use maud::PreEscaped;

pub fn show(portfolio: Portfolio) -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        article.portfolio id=^(format!("portfolio_{}", portfolio.id)) {

            @for portfolio in portfolios {

                div.portfolio__test {
                    span "test:"
                    span ^(portfolio.test)
                }

                div.portfolio__bob {
                    span "bob:"
                    span ^(portfolio.bob)
                }

            }

            ul.portfolio__actions.actions {
                li a href=^(format!("/portfolio/{}/edit", portfolio.id)) "Edit Portfolio"
                li a href="/portfolios" "View All"
            }
        }
    }).unwrap();

    PreEscaped(body)
}
