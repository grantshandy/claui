use claui::clap::Command;

fn main() {
    let command = Command::new("Panic Example");

    claui::run(command, |_| {
        panic!("This is a panic message.");
    });
}
