#![windows_subsystem = "windows"]

use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[clap(
    name = "Derive Greeter",
    author = "Grant Handy <grantshandy@gmail.com>",
    version = "1.2.3",
    about = "A derive example for clui"
)]
struct Args {
    #[clap(short, long, default_value = "Joe", help = "Your name")]
    name: String,
    #[clap(short, long, help = "Say goodbye")]
    goodbye: bool,
}

fn main() {
    let app = Args::command();

    clui::run(app, |matches| {
        println!("Hello, {}!", matches.value_of("name").unwrap());

        if matches.is_present("goodbye") {
            println!("Goodbye!");
        }
    });
}
