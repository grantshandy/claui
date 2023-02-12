use clap::{Arg, Command};

#[derive(Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub name: String,
    pub version: Option<String>,
    pub about: Option<String>,
    pub long_about: Option<String>,
    pub author: Option<String>,
}

impl AppInfo {
    pub fn new(app: &Box<Command>) -> Self {
        let name = app.get_name().to_string();
        let version = app.get_version().map(|x| x.to_string());
        let about = app.get_about().map(|x| x.to_string());
        let long_about = app.get_long_about().map(|x| x.to_string());
        let author = app.get_author().map(|x| x.to_string());

        Self {
            name,
            version,
            about,
            long_about,
            author,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgState {
    pub name: String,
    pub display_name: String,
    pub desc: Option<String>,
    pub default_value: Option<String>,
    pub takes_value: bool,
}

impl ArgState {
    pub fn new(arg: &Arg) -> Self {
        let default_value = arg
            .get_default_values()
            .get(0)
            .map(|x| x.to_str().unwrap().to_string());

        Self {
            name: arg.get_id().to_string(),
            display_name: capitalize(arg.get_id().as_str()),
            desc: arg.get_help().map(|h| h.to_string()),
            default_value,
            takes_value: arg.get_action().takes_values(),
        }
    }
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
        None => String::new(),
    }
}
