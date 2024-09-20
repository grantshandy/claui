#![doc = include_str!("../README.md")]

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
use eframe::{CreationContext, NativeOptions};
use misc::{AppInfo, ArgState};
use shh::{ShhStderr, ShhStdout};

pub use clap;
pub use eframe;

/// Run a clap [`Command`] as a GUI
pub fn run(
    app: Command,
    on_run: impl Fn(&ArgMatches) + Send + Sync + 'static,
) -> Result<(), eframe::Error> {
    run_setup(app, NativeOptions::default(), |_| {}, on_run)
}

/// Run a clap [`Command`] as a GUI and modify the GUI context on start.
pub fn run_setup(
    app: Command,
    eframe_options: NativeOptions,
    setup: impl FnOnce(&CreationContext<'_>) + Send + Sync + 'static,
    on_run: impl Fn(&ArgMatches) + Send + Sync + 'static,
) -> Result<(), eframe::Error> {
    eframe::run_native(
        app.clone().get_name(),
        eframe_options,
        Box::new(|cc| {
            setup(cc);

            Ok(Box::new(Claui::new(app, Arc::new(on_run))))
        }),
    )
}

type SharedFunction = Arc<dyn Fn(&ArgMatches) + Send + Sync + 'static>;

type SharedFunctionSender = Sender<(SharedFunction, ArgMatches)>;
type SharedFunctionReceiver = Receiver<(SharedFunction, ArgMatches)>;

struct Claui {
    app: Box<Command>,
    app_info: AppInfo,
    shh: (ShhStdout, ShhStderr),
    buffer: String,
    func: SharedFunction,
    func_handle: Option<Arc<JoinHandle<()>>>,
    args: Vec<ArgState>,
    ui_arg_state: HashMap<String, (bool, String)>,
}

impl Claui {
    pub fn new(app: Command, func: SharedFunction) -> Self {
        let app = Box::new(app);
        let app_info = AppInfo::new(&app);

        let mut args = Vec::new();
        for arg in app.get_arguments() {
            match arg.get_id().as_str() {
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

        let (sender, receiver): (SharedFunctionSender, SharedFunctionReceiver) = mpsc::channel();

        let matches = match self.app.clone().try_get_matches_from(self.get_arg_output()) {
            Ok(res) => res,
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        };

        self.func_handle = Some(Arc::new(
            thread::Builder::new()
                .name(String::from("claui child"))
                .spawn(move || {
                    let (func, matches) = receiver.recv().unwrap();

                    func(&matches);
                })
                .unwrap(),
        ));

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
                if !value.is_empty() {
                    res.push(format!("--{}", arg.name));
                    res.push(value);
                }
            } else if self.ui_arg_state.get(&arg.name.clone()).unwrap().0 {
                res.push(format!("--{}", arg.name));
            }
        }

        res
    }

    fn update_thread_state(&mut self) {
        if let Some(func_handle) = &self.func_handle {
            if func_handle.is_finished() {
                self.func_handle = None;
            }
        }
    }
}
