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
    } else {
        Config::display();
    }
}
