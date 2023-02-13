#![windows_subsystem = "windows"]

use std::{thread, time::Duration};

use clap::{ArgMatches, CommandFactory, Parser};

#[derive(Parser, Debug)]
#[clap(
    name = "Fizz Buzz",
    author = "Grant Handy <grantshandy@gmail.com>",
    version = "1.2.3",
    about = "An example implementation of FizzBuzz for claui",
    long_about = r#"Fizz buzz is a group word game for children to teach them about division. Players take turns to count incrementally, replacing any number divisible by three with the word "fizz", and any number divisible by five with the word "buzz". This program plays the game on its own, printing out every number and playing along to the rules."#
)]
struct Args {
    #[clap(
        short,
        long,
        default_value = "3",
        help = "Number to divide by for fizz"
    )]
    fizz: usize,
    #[clap(
        short,
        long,
        default_value = "5",
        help = "Number to divide by for buzz"
    )]
    buzz: usize,
    #[clap(short, long, default_value = "100", help = "Number to count to")]
    number: usize,
    #[clap(
        short,
        long,
        default_value = "100",
        help = "Milisecond gap between printing numbers"
    )]
    gap: u64,
    #[clap(short, long, help = "Print all lines with their number")]
    verbose: bool,
}

fn main() {
    let app = Args::command();

    claui::run(app, run).unwrap();
}

fn run(matches: &ArgMatches) {
    let number: usize = *matches.get_one("number").unwrap();
    let fizz_num: usize = *matches.get_one("fizz").unwrap();
    let buzz_num: usize = *matches.get_one("buzz").unwrap();
    let gap: u64 = *matches.get_one("gap").unwrap();

    let verbose: bool = matches.get_flag("verbose");

    for num in 1..(number + 1) {
        let mut output = String::new();

        if verbose {
            output += format!("{num}: ").as_str();
        }

        if num % fizz_num == 0 {
            output += "Fizz ";
        }

        if num % buzz_num == 0 {
            output += "Buzz";
        }

        if num % fizz_num != 0 && num % buzz_num != 0 {
            output += num.to_string().as_str();
        }

        if output != "" {
            println!("{output}");
        } else {
            println!("{num}");
        }

        thread::sleep(Duration::from_millis(gap));
    }
}
