use maud::PreEscaped;

pub fn index() -> PreEscaped<String> {
    let mut body = String::new();

    html!(body, {
        header {
            h1 { "Rusty Rails" span "is ready to go!" }
        }
        section {
            h2 "Getting started"
            ol {
                li { "Use " code { "generate scaffold" } " to create a RESTful resource." }
                li { "Use " code { "task assets" } " to compile SCSS and CommonJS JavaScript." }
                li { "Create and update the database using " code { "diesel migrate" } "." }
                li { "Access the " a href="/_rusty" { "built in analytics dashboard" } "." }
            }
        }
        section {
            h3 "Additional Help"
            figure {
                svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 510 510" style="height: 200pt; float: right; margin: 1rem;" {
                    path xmlns="http://www.w3.org/2000/svg" d="M255,0C114.75,0,0,114.75,0,255s114.75,255,255,255s255-114.75,255-255S395.25,0,255,0z M318.75,204l-107.1,107.1L153,255    l-84.15,84.15C58.65,313.65,51,285.6,51,255c0-112.2,91.8-204,204-204s204,91.8,204,204c0,25.5-5.1,51-15.3,73.95L318.75,204z" fill="#333333" /
                }
            }
            p  {
                "If this is your first time experiencing Rusty Rails, additional tutorials are available at "
                    a href="http://www.rusty-rails.com/tutorials" "Rusty-Rails.com"
            }
            p "A few topics to highlight are:"
            ol {
                li a href="#" "Version Control using Git"
                li a href="#" "Relational Databases"
                li a href="#" "Managing Front End Assets"
                li a href="#" "Internetworking and APIs"
            }
        }
        section {
            figure {
                svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" style="height: 200pt; float: right; margin-top: 3rem;" {
                    path d="M512 0C229.25 0 0 229.25 0 512c0 226.25 146.688 418.125 350.156 485.812 25.594 4.688 34.938-11.125 34.938-24.625 0-12.188-0.469-52.562-0.719-95.312C242 908.812 211.906 817.5 211.906 817.5c-23.312-59.125-56.844-74.875-56.844-74.875-46.531-31.75 3.53-31.125 3.53-31.125 51.406 3.562 78.47 52.75 78.47 52.75 45.688 78.25 119.875 55.625 149 42.5 4.654-33 17.904-55.625 32.5-68.375C304.906 725.438 185.344 681.5 185.344 485.312c0-55.938 19.969-101.562 52.656-137.406-5.219-13-22.844-65.094 5.062-135.562 0 0 42.938-13.75 140.812 52.5 40.812-11.406 84.594-17.031 128.125-17.219 43.5 0.188 87.312 5.875 128.188 17.281 97.688-66.312 140.688-52.5 140.688-52.5 28 70.531 10.375 122.562 5.125 135.5 32.812 35.844 52.625 81.469 52.625 137.406 0 196.688-119.75 240-233.812 252.688 18.438 15.875 34.75 47 34.75 94.75 0 68.438-0.688 123.625-0.688 140.5 0 13.625 9.312 29.562 35.25 24.562C877.438 930 1024 738.125 1024 512 1024 229.25 794.75 0 512 0z" fill="#333333" /
                }
            }
            h3 "Contributing"
            ul {
                li a href="http://github.com/jadencarver/rusty-rails" "Github Repository"
                li a href="#" "Documentation"
                li a href="#" "Known Issues"
                li a href="#" "Additional Support"
            }
        }
        footer ul {
            p "© 2016 Jaden Carver"
        }
    }).unwrap();

    PreEscaped(body)
}
