use crate::config::Config;

pub fn config_main(edit: &bool, path: &bool, key: &Option<String>) {
    if *edit {
        Config::edit();
    } else if *path {
        Config::display_path();
    } else if let Some(key) = key {
        Config::display_value(key);
    } else {
        Config::display();
    }
}
