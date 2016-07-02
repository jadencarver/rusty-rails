use maud::PreEscaped;

pub fn portfolio() -> PreEscaped<String> {
	let mut body = String::new();

	html!(body, {
        article id="portfolio" {
            header a href="/resume" h1 "Jaden Carver"
            section {
                h3 "Corporate Projects"
                ul {
                    li a href="/portfolio/ga-teachr"          "GA Teachr"
                    li a href="/portfolio/ga-quiz"            "GA Quiz"
                    li a href="/portfolio/ga-simonsays"       "Simon Says / Red Rover"
                    li a href="/portfolio/ga-upillar"         "Upillar.com"
                }
            }
            section {
                h3 "Educational Material"
                ul {
                    li a href="/portfolio/squids-flounders"   "Squids & Flounders"
                    li a href="/portfolio/pc-logo"            "PC Logo"
                    li a href="/portfolio/duck-hunt"          "Upillar.com"
                    li a href="/portfolio/megaman"            "Megaman"
                }
            }
        }
    }).unwrap();

	PreEscaped(body)
}
