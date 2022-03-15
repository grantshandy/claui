#![windows_subsystem = "windows"]

use clap::Command;

fn main() {
    let app = Command::new("Basic");

    claui::run(app, |_| {
        println!("Hello, World!");
    });
}
