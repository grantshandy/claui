#![windows_subsystem = "windows"]

use std::{thread, time::Duration};

use clap::{arg, Command};

fn main() {
    let app = Command::new("FizzBuzz")
        .author("Grant Handy <grantshandy@gmail.com>")
        .version("1.2.3")
        .about("An example program to demonstrate clui")
        .long_about(r#"Fizz buzz is a group word game for children to teach them about division. Players take turns to count incrementally, replacing any number divisible by three with the word "fizz", and any number divisible by five with the word "buzz". This program plays the game on its own, printing out every number and playing along to the rules."#)
        .arg(arg!(-f --fizz "Number to divide by for fizz").takes_value(true).default_value("3"))
        .arg(arg!(-b --buzz "Number to divide by for buzz").takes_value(true).default_value("5"))
        .arg(arg!(-n --number "Number to count to").takes_value(true).default_value("100"))
        .arg(arg!(-v --verbose "Print all lines with their number"));

    clui::run(app, |matches| {
        let number = matches
            .value_of("number")
            .unwrap_or("100")
            .parse::<usize>()
            .expect("number must be a number!");

        for x in 1..(number + 1) {
            let mut s = String::new();

            let fizz_num = matches
                .value_of("fizz")
                .unwrap_or("3")
                .parse::<usize>()
                .expect("fizz must be a positive integer");
            let buzz_num = matches
                .value_of("buzz")
                .unwrap_or("5")
                .parse::<usize>()
                .expect("buzz must be a positive integer");

            if x % fizz_num == 0 {
                s += "Fizz ";
            }

            if x % buzz_num == 0 {
                s += "Buzz";
            }

            if matches.is_present("verbose") {
                if s != "" {
                    println!("{x}: {s}");
                } else {
                    println!("{x}: {x}");
                }
            } else {
                if s != "" {
                    println!("{s}");
                } else {
                    println!("{x}");
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
}
