use std::process::Command;
use glob::glob;

pub fn compile() {
    for file in glob("app/assets/*.scss").unwrap() {
        let path = file.unwrap();
        let src = path.to_str().unwrap();
        let dest = format!("public/assets/{}.css", path.file_stem().unwrap().to_str().unwrap());
        Command::new("scss")
            .arg(src).arg(dest.clone())
            .arg("--cache-location").arg("tmp/cache")
            .arg("--load-path").arg("app/assets/stylesheets")
            .args(&["--style","compressed"])
            .status().expect("Unable to run the scss command.");
        println!("- compiled {}", dest);
    }
}
