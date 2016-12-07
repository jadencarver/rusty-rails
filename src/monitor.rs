extern crate termion;
extern crate notify;
extern crate glob;
extern crate ws;

use termion::{color, style, clear};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use ws::{listen};
mod tasks;

const ESC: char = 27 as char;

fn main() {
    let (watcher_tx, watcher_rx) = channel();
    let (server_tx, server_rx) = channel();
    let (builder_tx, builder_rx) = channel();
    let (js_assets_tx, js_assets_rx) = channel();
    let (css_assets_tx, css_assets_rx) = channel();
    //let (reload_tx, reload_rx) = channel();
    let server_path = Path::new("./target/debug/server");

    // Initial Build
    builder_tx.send(()).unwrap();
    js_assets_tx.send(()).unwrap();
    css_assets_tx.send(()).unwrap();

    let watcher = thread::spawn(move || {
        let mut watcher_fs: RecommendedWatcher = Watcher::new(watcher_tx, Duration::from_secs(2)).unwrap();
        watcher_fs.watch("./app", RecursiveMode::Recursive).unwrap();
        loop {
            let event = watcher_rx.recv().unwrap();
            let path: Option<PathBuf> = match event {
              _ => None
            };
            if let Some(path) = path {
              if let Some(channel) = match path.extension() {
                  Some(ext) if ext == "rs" => Some(&builder_tx),
                  Some(ext) if ext == "scss" => Some(&css_assets_tx),
                  Some(ext) if ext == "js" => Some(&js_assets_tx),
                  _ => None
              } {
                  let fg = color::Fg(color::Rgb(0xEE,0xEE,0xEE));
                  let bg = color::Bg(color::Rgb(0x33,0x33,0x33));
                  let (width, height) = termion::terminal_size().unwrap_or((80,25));
                  println!("{}{}{}{}", bg, fg, format!("―――{:―<1$}", format!(" {} ", path.to_str().unwrap()), (width - 3) as usize), style::Reset);
                  channel.send(()).unwrap()
              }
            }
        }
    });

    let builder = thread::spawn(move || {
        loop {
            builder_rx.recv().unwrap();
            thread::sleep(Duration::from_millis(100));
            'flush: loop {
                match builder_rx.try_recv() {
                    Ok(_) => {}
                    Err(_) => break 'flush
                }
            }
            let build = Command::new("cargo").arg("build").arg("--bin").arg("server").status();
            if build.unwrap().success() {
                server_tx.send(()).unwrap();
            }
        }
    });

    let js_assets = thread::spawn(move || {
        loop {
            js_assets_rx.recv().unwrap();
            thread::sleep(Duration::from_millis(100));
            'flush: loop {
                match js_assets_rx.try_recv() {
                    Ok(_) => {},
                    Err(_) => break 'flush
                }
            }
            tasks::javascripts::compile();
        }
    });

    let css_assets = thread::spawn(move || {
        loop {
            css_assets_rx.recv().unwrap();
            tasks::stylesheets::compile();
            //reload_tx.send(());
        }
    });

    let server = thread::spawn(move || {
        loop {
            print!("{}[H{}[2J", ESC, ESC);
            println!("{}Starting{} Rusty Rails", color::Fg(color::Green), style::Reset);
            match Command::new(server_path).spawn() {
                Ok(mut handle) => {
                    server_rx.recv().unwrap();
                    handle.kill().unwrap();
                    handle.wait().unwrap();
                }
                Err(msg) => {
                    println!("    {}Unable to start server{} {}", color::Fg(color::Red), style::Reset, msg);
                    server_rx.recv().unwrap();
                }
            }
        }
    });

    // TODO: Reload Sockets
    //listen("127.0.0.1:35729", |out| {
    //    reload_rx.recv().unwrap();
    //    println!("sending css");
    //    out.send("css");
    //    move |msg| {
    //        out.send(msg)
    //    }
    //});

    watcher.join().unwrap();
    builder.join().unwrap();
    js_assets.join().unwrap();
    css_assets.join().unwrap();
    server.join().unwrap();
}
