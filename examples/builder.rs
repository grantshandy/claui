#![windows_subsystem = "windows"]

use clap::{arg, Command};

fn main() {
    let app = Command::new("Builder Greeter")
        .author("Grant Handy <grantshandy@gmail.com>")
        .version("1.2.3")
        .about("A builder example for claui")
        .arg(arg!(--name [NAME] "Your name").default_value("Joe"))
        .arg(arg!(--goodbye "Say goodbye"));

    claui::run(app, |matches| {
        println!("Hello, {}!", matches.get_one::<String>("name").unwrap());

        if matches.get_flag("goodbye") {
            println!("Goodbye!");
        }
    })
    .unwrap();
}
