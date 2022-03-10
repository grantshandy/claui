use clap::{arg, Command};

fn main() {
    let app = Command::new("Example Program")
        .author("Me, <me@mail.com>")
        .version("1.2.3")
        .about("This is an example program to demonstrate clui")
        .arg(arg!(-t --test "used for testing arguments").takes_value(true))
        .arg(arg!(-s --secondtest "second arg used for testing arguments"));

    clui::run(app, |matches| {
        println!("Example Program...\n{:#?}", matches);
    });
}
