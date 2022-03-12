use clap::Command;

fn main() {
    let app = Command::new("Basic");

    clui::run(app, |matches| {
        println!("{:#?}", matches);
    });
}