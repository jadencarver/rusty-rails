use std::process::Command;
use glob::glob;
use std::fs::*;

pub fn compile() {
    create_dir("tmp").unwrap_or(());
    create_dir("tmp/cache").unwrap_or(());
    create_dir("public").unwrap_or(());
    create_dir("public/assets").unwrap_or(());
    for file in glob("app/assets/*.scss").unwrap() {
        let path = file.unwrap();
        let src = path.to_str().unwrap();
        let filename = path.file_stem().unwrap().to_str().unwrap();
        let dest = format!("{}.css", filename);
        let dest_map = format!("{}.css.map", filename);
        let final_dest = format!("public/assets/{}.css", filename);
        let final_dest_map = format!("public/assets/{}.css.map", filename);
        Command::new("scss")
            .arg(src).arg(dest.clone())
            .arg("--cache-location").arg("tmp/cache")
            .arg("--load-path").arg("app/assets/stylesheets")
            .args(&["--style","compressed"])
            .status().expect("Unable to run the scss command.");
        Command::new("mv").arg(dest).arg(final_dest.clone()).status().expect("Unable to move compiled css");
        Command::new("mv").arg(dest_map).arg(final_dest_map).status().expect("Unable to move compiled css.map");
        println!("- compiled {}", final_dest);
    }
}
