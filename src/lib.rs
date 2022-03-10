use std::{io::Read, thread};

use clap::{ArgMatches, Command};
use crossbeam_channel::{Receiver, Sender};
use eframe::{
    egui::{self, CentralPanel, Context, RichText, ScrollArea, TextStyle},
    epi,
};
use shh::ShhStdout;

pub fn run(app: Command<'static>, func: Box<dyn Fn(&ArgMatches) + Send + Sync>) -> ! {
    let func = Box::new(func);
    println!("Initializing clui");

    eframe::run_native(
        Box::new(Clui::new(app, func)),
        eframe::NativeOptions::default(),
    )
}

type SharedFunction = Box<dyn Fn(&ArgMatches) + Send + Sync + 'static>;

struct Clui {
    name: String,
    version: Option<String>,
    about: Option<String>,
    shh: ShhStdout,
    buffer: String,
    func: SharedFunction,
    app: Box<Command<'static>>,
}

impl Clui {
    pub fn new(app: Command<'static>, func: SharedFunction) -> Self {
        let app = Box::new(app);
        let shh = shh::stdout().unwrap();
        let buffer = String::new();

        let name = app.get_name().to_string();
        let version = match app.get_version() {
            Some(version) => Some(version.to_string()),
            None => None,
        };
        let about = match app.get_about() {
            Some(about) => Some(about.to_string()),
            None => None,
        };

        Self {
            name,
            version,
            about,
            shh,
            buffer,
            func,
            app,
        }
    }

    fn update_buffer(&mut self) {
        self.shh.read_to_string(&mut self.buffer).unwrap();
    }

    fn run(&mut self) {
        self.buffer.clear();

        let (sender, reciever): (
            Sender<(SharedFunction, ArgMatches)>,
            Receiver<(SharedFunction, ArgMatches)>,
        ) = crossbeam_channel::unbounded();
        let matches = self.app.clone().get_matches();

        let handle = thread::spawn(move || {
            let (func, matches) = reciever.recv().unwrap();

            func(&matches);
        });

        sender.send((self.func, matches));
        handle.join();
    }
}

impl epi::App for Clui {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        self.update_buffer();

        CentralPanel::default().show(ctx, |ui| {
            // title and description
            ui.horizontal(|ui| {
                ui.heading(&self.name);
                if let Some(version) = &self.version {
                    ui.label(RichText::new(version));
                }
            });
            if let Some(about) = &self.about {
                ui.label(about);
            }
            ui.add_space(4.5);

            ui.horizontal(|ui| {
                // Run button
                if ui.button("Run").clicked() {
                    self.run();
                }

                if ui.button("Show State").clicked() {
                    self.buffer.clear();

                    println!("{:#?}", self.app);
                }
            });

            ui.separator();

            // The results for running the text
            ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.buffer)
                        .interactive(false)
                        .font(TextStyle::Monospace),
                );
            });
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
        frame.request_repaint();
    }
}
