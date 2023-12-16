use crate::commands::ConfigKey;
use crate::config::Config;

pub fn main(
    edit: bool,
    path: bool,
    key: &Option<ConfigKey>,
    set: &Option<String>,
) {
    if edit {
        Config::edit();
    } else if path {
        Config::display_path();
    } else if let Some(key) = key {
        set.as_ref().map_or_else(
            || {
                Config::display_value(key);
            },
            |value| Config::set_value(key, value.to_string()),
        );
    } else if let Some(set) = set {
        if let Some(key) = key {
            Config::set_value(key, set.to_string());
        } else {
            println!("Please specify the key to set.");
        }
    } else {
        Config::display();
    }
}
