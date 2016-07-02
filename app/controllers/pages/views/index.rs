extern crate rand;

use maud::PreEscaped;
use std::num;
use self::rand::Rng;

pub fn index() -> PreEscaped<String> {
	let mut body = String::new();
    let mut rng = rand::thread_rng();

    html!(body, {
        svg id="tessellation" width="100%" height="100%" viewbox="-1000 -1000 2000 2000" {
            @for yi in -4..5 {
            @for xi in -5..6 {
                rect class=^(format!("tessellation-sq {}", if rng.gen() {"on"} else {""})) x=^(-100 + (xi * 300) + (yi * 100)) y=^(-100 + (xi *  100) + (yi * 300)) width=^(200) height=^(200) {}
                rect class=^(format!("tessellation-rt {}", if rng.gen() {"on"} else {""})) x=^( 100 + (xi * 300) + (yi * 100)) y=^(-300 + (xi * -200) + (yi * 200)) width=^(100) height=^(200) {}
                rect class=^(format!("tessellation-bt {}", if rng.gen() {"on"} else {""})) x=^(-300 + (xi * 200) + (yi *-200)) y=^( 100 + (xi *  100) + (yi * 300)) width=^(200) height=^(100) {}
            }
            }
            text class="name" x=0 y=400 font-size="1000pt" text-anchor="middle" "JC"
        }
    }).unwrap();

    PreEscaped(body)
}
