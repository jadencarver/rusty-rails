extern crate ansi_term;
extern crate notify;
extern crate glob;

use ansi_term::Colour::*;
use notify::{RecommendedWatcher, Watcher};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
mod tasks;

fn main() {
    let (watcher_tx, watcher_rx) = channel();
    let (server_tx, server_rx) = channel();
    let (builder_tx, builder_rx) = channel();
    let (js_assets_tx, js_assets_rx) = channel();
    let (css_assets_tx, css_assets_rx) = channel();

    let watcher = thread::spawn(move || {
        let mut watcher_fs: RecommendedWatcher = Watcher::new(watcher_tx).unwrap();
        watcher_fs.watch("./app").unwrap();
        loop {
            let event = watcher_rx.recv().unwrap();
            let path = event.path.unwrap();
            match path.extension() {
                Some(ext) if ext == "rs" => builder_tx.send(()).unwrap(),
                Some(ext) if ext == "scss" => css_assets_tx.send(()).unwrap(),
                Some(ext) if ext == "js" => js_assets_tx.send(()).unwrap(),
                _ => {}
            }
        }
    });

    let builder = thread::spawn(move || {
        loop {
            builder_rx.recv().unwrap();
            thread::sleep(Duration::from_millis(50));
            'flush: loop {
                match builder_rx.try_recv() {
                    Ok(_) => {}
                    Err(_) => break 'flush
                }
            }
            println!("{}[2J", 27 as char);
            Command::new("cargo").arg("build").arg("--bin").arg("server").status().unwrap();
            server_tx.send(()).unwrap();
        }
    });

    let js_assets = thread::spawn(move || {
        loop {
            js_assets_rx.recv().unwrap();
            tasks::javascripts::compile();
        }
    });

    let css_assets = thread::spawn(move || {
        loop {
            css_assets_rx.recv().unwrap();
            tasks::stylesheets::compile();
        }
    });

    let server = thread::spawn(move || {
        loop {
            println!("{} Rusty Rails", Green.bold().paint("Starting"));
            match Command::new("./target/debug/server").spawn() {
                Ok(mut handle) => {
                    server_rx.recv().unwrap();
                    handle.kill().unwrap();
                    handle.wait().unwrap();
                }
                Err(msg) => {
                    println!("    {} {}", Red.bold().paint("Unable to start server"), msg);
                    server_rx.recv().unwrap();
                }
            }
        }
    });

    watcher.join().unwrap();
    builder.join().unwrap();
    js_assets.join().unwrap();
    css_assets.join().unwrap();
    server.join().unwrap();
}
