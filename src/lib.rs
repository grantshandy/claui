//! # claui
//! *Command Line Argument (to graphical) User Interface*
//!
//! A GUI generator for [`clap`](https://github.com/clap-rs/clap) using [`egui`](https://github.com/emilk/egui).
//!
//! ## Builder Example
//! ```rust
//! use clap::{arg, Command};
//!
//! fn main() {
//!     let app = Command::new("Builder Greeter")
//!         .author("Grant Handy <grantshandy@gmail.com>")
//!         .version("1.2.3")
//!         .about("A builder example for claui")
//!         .arg(arg!(--name "Your name").default_value("Joe"))
//!         .arg(arg!(--goodbye "Say goodbye"));
//!
//!     claui::run(app, |matches| {
//!         println!("Hello, {}!", matches.value_of("name").unwrap());
//!
//!         if matches.is_present("goodbye") {
//!             println!("Goodbye!");
//!         }
//!     });
//! }
//! ```
//!
//! ## Derive Example
//! ```rust
//! use clap::{CommandFactory, Parser};
//!
//! #[derive(Parser, Debug)]
//! #[clap(
//!     name = "Derive Greeter",
//!     author = "Grant Handy <grantshandy@gmail.com>",
//!     version = "1.2.3",
//!     about = "A derive example for claui"
//! )]
//! struct Args {
//!     #[clap(long, default_value = "Joe", help = "Your name")]
//!     name: String,
//!     #[clap(long, help = "Say goodbye")]
//!     goodbye: bool,
//! }
//!
//! fn main() {
//!     let app = Args::command();
//!
//!     claui::run(app, |matches| {
//!         println!("Hello, {}!", matches.value_of("name").unwrap());
//!
//!         if matches.is_present("goodbye") {
//!             println!("Goodbye!");
//!         }
//!     });
//! }
//! ```
//!
//! ## Comparison with [`klask`](https://github.com/MichalGniadek/klask)
//! Klask is another GUI generator for [`clap`](https://github.com/clap-rs/clap) that uses [`egui`](https://github.com/emilk/egui), but claui and klask work in different ways. Klask runs your code by running itself as a child with an environment variable to ignore its GUI, then capturing the child stdout. Claui only runs one process; it spawns your code in another thread and then reroutes all of your stdout into a buffer on each frame through [`shh`](https://github.com/kurtlawrence/shh).

#![feature(thread_is_running)]

use std::{
    collections::HashMap,
    env,
    io::Read,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

mod misc;
mod ui;

use clap::{ArgMatches, Command};
use misc::{AppInfo, ArgState};
use shh::{ShhStderr, ShhStdout};

/// Run a clap [`Command`](egui::Command) as a GUI
pub fn run<F: Fn(&ArgMatches) + Send + Sync + 'static>(app: Command<'static>, func: F) -> ! {
    eframe::run_native(
        Box::new(Claui::new(app, Arc::new(func))),
        eframe::NativeOptions::default(),
    )
}

type SharedFunction = Arc<dyn Fn(&ArgMatches) + Send + Sync + 'static>;

struct Claui {
    app: Box<Command<'static>>,
    app_info: AppInfo,
    shh: (ShhStdout, ShhStderr),
    buffer: String,
    func: SharedFunction,
    func_handle: Option<Arc<JoinHandle<()>>>,
    is_running: bool,
    args: Vec<ArgState>,
    ui_arg_state: HashMap<String, (bool, String)>,
}

impl Claui {
    pub fn new(app: Command<'static>, func: SharedFunction) -> Self {
        let app = Box::new(app);
        let app_info = AppInfo::new(&app);

        let mut args = Vec::new();
        for arg in app.get_arguments() {
            match arg.get_id() {
                "version" => (),
                "help" => (),
                _ => args.push(ArgState::new(arg)),
            }
        }

        let mut ui_arg_state = HashMap::new();
        for arg in &args {
            ui_arg_state.insert(arg.name.clone(), (false, String::new()));
        }

        Self {
            app,
            app_info,
            shh: (shh::stdout().unwrap(), shh::stderr().unwrap()),
            buffer: String::new(),
            func,
            func_handle: None,
            is_running: false,
            args,
            ui_arg_state,
        }
    }

    fn update_buffer(&mut self) {
        self.shh.0.read_to_string(&mut self.buffer).unwrap();
        self.shh.1.read_to_string(&mut self.buffer).unwrap();
    }

    fn run(&mut self) {
        self.buffer.clear();

        let (sender, receiver): (
            Sender<(SharedFunction, ArgMatches)>,
            Receiver<(SharedFunction, ArgMatches)>,
        ) = mpsc::channel();

        let matches = match self.app.clone().try_get_matches_from(self.get_arg_output()) {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        };

        let func_handle = thread::Builder::new()
            .name(String::from("claui child"))
            .spawn(move || {
                let (func, matches) = receiver.recv().unwrap();

                func(&matches);
            })
            .unwrap();

        self.func_handle = Some(Arc::new(func_handle));
        self.is_running = true;

        sender.send((Arc::clone(&self.func), matches)).unwrap();
    }

    fn get_arg_output(&mut self) -> Vec<String> {
        let mut res = Vec::new();

        res.push(
            env::current_exe()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_string(),
        );

        for arg in self.args.iter() {
            if arg.takes_value {
                let value = self
                    .ui_arg_state
                    .get(&arg.name.clone())
                    .unwrap()
                    .1
                    .to_owned();
                if value != "" {
                    res.push(format!("--{}", arg.name));
                    res.push(value);
                }
            } else {
                if self.ui_arg_state.get(&arg.name.clone()).unwrap().0 {
                    res.push(format!("--{}", arg.name));
                }
            }
        }

        res
    }

    fn update_thread_state(&mut self) {
        if let Some(func_handle) = &self.func_handle {
            if func_handle.is_finished() {
                self.func_handle = None;
                self.is_running = false;
            }
        }
    }
}
