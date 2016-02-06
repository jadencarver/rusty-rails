extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
  let mut command = String::new();
  {  // this block limits scope of borrows by ap.refer() method
    let mut ap = ArgumentParser::new();
    ap.set_description("Code generation tool for Rapid Application Development");
    ap.refer(&mut command).add_argument("generator", Store, "Generator to Run");
    ap.parse_args_or_exit();
  }

  match command.as_ref() {
    "scaffold" => println!("Scaffold"),
    _ => println!("Unknown command {}", command)
  }
}