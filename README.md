# claui
*Command Line Arguments (to graphical) User Interface*

A GUI generator for [`clap`](https://github.com/clap-rs/clap) that uses [`egui`](https://github.com/emilk/egui).

![fizzbuzz screenshot](https://github.com/grantshandy/claui/blob/main/screenshots/fizzbuzz.png?raw=true)

## Builder Example
```rust
use clap::{arg, Command};

fn main() {
    let app = Command::new("Builder Greeter")
        .author("Grant Handy <grantshandy@gmail.com>")
        .version("1.2.3")
        .about("A builder example for claui")
        .arg(arg!(--name "Your name").default_value("Joe"))
        .arg(arg!(--goodbye "Say goodbye"));

    claui::run(app, |matches| {
        println!("Hello, {}!", matches.value_of("name").unwrap());

        if matches.is_present("goodbye") {
            println!("Goodbye!");
        }
    });
}
```
![builder screenshot](https://github.com/grantshandy/claui/blob/main/screenshots/builder.png?raw=true)

## Derive Example
```rust
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
        println!("Hello, {}!", matches.value_of("name").unwrap());

        if matches.is_present("goodbye") {
            println!("Goodbye!");
        }
    });
}
```
![derive example](https://github.com/grantshandy/claui/blob/main/screenshots/derive.png?raw=true)

## Comparison with [`klask`](https://github.com/MichalGniadek/klask)
Klask is another GUI generator for [`clap`](https://github.com/clap-rs/clap) that uses [`egui`](https://github.com/emilk/egui), but claui and klask work in different ways. Klask runs your code by running itself as a child with an environment variable to ignore its GUI, then capturing the child stdout. Claui only runs one process; it spawns your code in another thread and then reroutes all of your stdout into a buffer on each frame through [`shh`](https://github.com/kurtlawrence/shh).