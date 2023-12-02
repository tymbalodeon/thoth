use crate::commands::ConfigKey;
use crate::config::Config;

pub fn config_main(
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
        if let Some(value) = set {
            Config::set_value(key, value.to_string());
        } else {
            Config::display_value(key);
        }
    } else {
        Config::display();
    }
}
