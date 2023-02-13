#![windows_subsystem = "windows"]

use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[clap(
    name = "Derive Greeter",
    author = "Grant Handy <grantshandy@gmail.com>",
    version = "1.2.3",
    about = "A derive example for claui"
)]
struct Args {
    #[clap(long, default_value = "Joe", help = "Your name")]
    name: String,
    #[clap(long, help = "Say goodbye")]
    goodbye: bool,
}

fn main() {
    let app = Args::command();

    claui::run(app, |matches| {
        println!("Hello, {}!", matches.get_one::<String>("name").unwrap());

        if matches.get_flag("goodbye") {
            println!("Goodbye!");
        }
    })
    .unwrap();
}
