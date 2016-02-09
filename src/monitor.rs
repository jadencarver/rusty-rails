extern crate ansi_term;
extern crate notify;

use std::process::Command;
use std::thread;
use ansi_term::Colour::*;
use notify::{RecommendedWatcher, Error, Watcher};
use std::sync::mpsc::channel;

fn main() {
    let (watcher_tx, watcher_rx) = channel();
    let (server_tx, server_rx) = channel();
    let (builder_tx, builder_rx) = channel();

    let watcher = thread::spawn(move || {
        let mut watcher_fs: RecommendedWatcher = Watcher::new(watcher_tx).unwrap();
        watcher_fs.watch("./app");
        loop {
            match watcher_rx.recv() {
                Ok(event) => {
                    let path = event.path.unwrap();
                    match path.extension().unwrap().to_str().unwrap() {
                        "rs" => {
                            println!("Watcher {}", path.to_str().unwrap());
                            builder_tx.send(());
                        }
                        _ => {}
                    }
                }
                Err(msg) => println!("Watcher Error: {}", msg)
            };
        }
    });

    let builder = thread::spawn(move || {
        loop {
            match builder_rx.recv() {
                Ok(_) => {
                    Command::new("cargo").arg("build").arg("--bin").arg("server").status().unwrap();
                    server_tx.send(());
                    builder_rx.recv();
                }
                Err(msg) => println!("Server Error: {}", msg)
            }
        }
    });

    let server = thread::spawn(move || {
        loop {
            println!("{} Rusty Rails",Green.bold().paint("   Starting"));
            let mut handle = Command::new("cargo").arg("run").arg("--bin").arg("server").spawn().unwrap();
            'server: loop {
                match server_rx.recv() {
                    Ok(_) => {
                        println!("Server Restart");
                        handle.kill();
                        println!("Server killed");
                        break 'server;
                    }
                    Err(msg) => println!("Server Error: {}", msg)
                };
            }
        }
    });

    watcher.join().unwrap();
    builder.join().unwrap();
    server.join().unwrap();
}
