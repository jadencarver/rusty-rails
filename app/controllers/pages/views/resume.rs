use maud::PreEscaped;

pub fn resume() -> PreEscaped<String> {
	let mut body = String::new();

    struct Skill {
        start: f32,
        end: f32,
        label: &'static str,
    }

    let skills = vec![
        Skill { start: 1998.0, end: 2016.0, label: "HTML/CSS/JavaScript" },
        Skill { start: 2002.0, end: 2006.0, label: "PC Repair/Internetworking" },
        Skill { start: 2003.0, end: 2016.0, label: "Linux DevOps" },
        Skill { start: 2003.0, end: 2016.0, label: "RDBMS/SQL" },
        Skill { start: 2003.0, end: 2009.0, label: "PHP (LAMP)" },
        Skill { start: 2009.0, end: 2016.0, label: "Agile/XP/BDD)" },
        Skill { start: 2009.0, end: 2016.0, label: "Ruby (Rails)" },
        Skill { start: 2010.5, end: 2016.0, label: "JS+Frameworks" },
    ];

	html!(body, {
		article id="resume" {
            header {
                h1 "Jaden Carver"
                a href="/portfolio" {
                    figure {
                        figcaption "JC"
                        div "≡"
                    }
                }
            }
            section {
                h2 "Objective"
                p  "To explore new avenues of creativity and invention and to bring about positive effects on society."
            }
            section {
                h2 "Summary"
                p  "I'm a long time technology-obsessed dog lover, with extensive Linux, database and networking expertise. I am fascinated with machine learning and big data analytics to improve society and our collective human experience."
            }
            section {
                h2 {
                    "Experience"
                    span {
                        a.expander href="#" "Show All"
                        a.expander.collapsed href="#" "Hide Inapplicable"
                    }
                }
                ol {
                    li {
                        h4 { "Bloomberg L.P." em "New York, NY - 2016-present" }
                    }
                    li {
                        h4 { "General Assembly" em "New York, NY - 2015-2016" }
                        p  "Instructor for a 3 month Web Development Immersive Bootcamp.
                            Principally Ruby on Rails, JavaScript."
                        ul {
                            li "Improved student feedback by developing an application to give
                                assessments and efficiently grade answers."
                            li "Lessons included: Agile, Ruby on Rails, Databases (SQL/MongoDB),
                                JavaScript, React, Angular.js and Node.js."
                            li "Worked to introduce design elements into a program that was
                                otherwise predominately technical."
                        }
                    }
                    li {
                        h4 { "Simon & Schuster" em "New York, NY - 2012-2014" }
                        p "Agile/XP & Ruby on Rails Web Developer for the consumer facing e­commerce
                           website for the well known book publishing company."
                        ul {
                            li "Played a major role upgrading a large Rails 2 project to Rails 3."
                            li "Exported, combined, and transformed ~2.5mil subscribers from 6
                                databases with varying schemas in ~200 lines of SQL."
                            li "Facial recognition (OpenCV), cropped 4,000 high resolution author
                                photos, adapting to the location of the face for the aesthetic."
                            li "Designed a solution to merging data from a multiple data sources
                                with diverse schemas, complete with data conflict resolution,
                                error tolerance and data recovery features."
                            li "Designed a Service Oriented Architecture to help diversify their
                                product offerings."
                        }
                    }
                    li.collapsed.defunct {
                        h4 { "Lexon Industries" em "St. George, UT - 2009-2012" }
                        p  "Built and maintained a full-featured classifieds ad website."
                        ul {
                            li "Developed a flexible authentication system where Users have multiple
                                Agencies, enabling a single login to manage a personal account,
                                a business (multiple employees), and admin features and resources."
                            li "Developed a messaging system using Faye/XMPP/push for in­browser
                                multi­user chat between buyers and sellers, customer support,
                                and event notifications."
                            li "Used XMPP to push bids to visitors for the Glenn Beck auctions, and
                                later redesigned for use on any listing."
                        }
                    }
                    li.collapsed {
                        h4 { "Discovery Research Group" em "CATI Script Programmer" }
                        p  "Coded and manageddata for computer­assisted telephone interviewing
                            surveys and web­based surveys using CfMC and PHP."
                        ul {
                            li  "Introducing PHP to create CfMC code generating templates and
                                 automating the exporting of data for the HCAHPS hospital surveys,
                                 which reduced the overhead significantly over the manual process."
                        }
                    }
                    li {
                        h4 { "Internet Effects" em "St. George, UT - 2004-2006" }
                        p  "Worked with a variety of clients building websites using Bash, MySQL,
                            PHP, and a custom proprietary language known as AMP. Clients usually
                            required design work, merchant accounts & shopping carts."
                        ul {
                            li "Developed a hierarchical content management system used by
                                Zion Adventure Company that fully integrated their existing POS
                                software (OSX/Cocoa). Customers could also make reservations, and
                                purchase from their online store, all integrated."
                        }
                    }
                    li.collapsed {
                        h4 { "Stonefly Technologies" em "St. George, UT" }
                    }
                    li.collapsed.defunct {
                        h4 { "Alliance Business Partners" em "Front End Web Developer" }
                    }
                }
            }
            section id="tech_proficiencies" {
                h2 "Tech Proficiencies"
                svg viewbox="0 0 1000 300" width="100%" {
                    line x1="0" y1="250" x2="1000" y2="250" stroke="#333333" {}
                    text x="25"  y="275" text-anchor="start" { "1998" }
                    text x="975" y="275" text-anchor="end"   { "2016" }
                    @for i in 0..19 {
                        line x1=^(i as f64 * 55.55) y1="0" x2=^(i as f64 * 55.55) y2="250" stroke="#CCCCCC" {}
                    }
                    @for (i,skill) in skills.iter().enumerate() {
                        rect rx=4 x=^((skill.start - 1998.0) * 55.55) y=^(250/skills.len()*i) width=^((skill.end - skill.start) * 55.55) height="25" fill="#333333" {}
                        text x=^((skill.start - 1998.0) * 55.55 + 5.0) y=^(250/skills.len()*i+18) fill="#FFFFFF" { ^skill.label }
                    }
                }
                ul {
                    @for skill in skills.iter() {
                        li {
                            span ^(skill.label)
                            span  " — "
                            span  ^(format!("{} years ", skill.end - skill.start))
                            small ^(format!("({}-{})", skill.start.floor(), skill.end.ceil()))
                        }
                    }
                }
            }
            section {
                h2 "Education"
                ul {
                    li "General Assembly"
                    li "Dixie State College"
                    li "Snow College South"
                }
            }
		}
	}).unwrap();

	PreEscaped(body)
}
