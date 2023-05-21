use glob::glob;

use crate::config::Config;

pub fn list_pdfs() {
    let scores_directory = Config::new().scores_directory();
    let pattern = format!("{}/**/*.pdf", scores_directory);

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{}", path.display()),
            Err(message) => println!("{:?}", message),
        }
    }
}
