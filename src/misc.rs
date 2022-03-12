use clap::{Arg, Command};

#[derive(Clone, Debug, PartialEq)]
pub struct AppInfo {
    pub name: String,
    pub ver: Option<String>,
    pub about: Option<String>,
    pub long_about: Option<String>,
    pub author: Option<String>,
}

impl AppInfo {
    pub fn new(app: &Box<Command>) -> Self {
        let name = app.get_name().to_string();
        let ver = match app.get_version() {
            Some(version) => Some(version.to_string()),
            None => None,
        };
        let about = match app.get_about() {
            Some(about) => Some(about.to_string()),
            None => None,
        };
        let long_about = match app.get_long_about() {
            Some(about) => Some(about.to_string()),
            None => None,
        };
        let author = match app.get_author() {
            Some(author) => Some(author.to_string()),
            None => None,
        };

        Self {
            name,
            ver,
            about,
            long_about,
            author,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgState {
    pub name: String,
    pub desc: Option<String>,
    pub default_value: Option<String>,
    pub required: bool,
    pub takes_value: bool,
}

impl ArgState {
    pub fn new(arg: &Arg) -> Self {
        let default_value = match arg.get_default_values().get(0) {
            Some(s) => Some(s.to_str().unwrap().to_string()),
            None => None,
        };

        Self {
            name: arg.get_id().to_string(),
            desc: arg.get_help().map(|h| h.to_string()),
            default_value,
            required: arg.is_required_set(),
            takes_value: arg.is_takes_value_set(),
        }
    }
}
