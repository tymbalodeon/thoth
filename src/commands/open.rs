use std::process::Command;

use glob::glob;

use crate::commands::patterns::get_patterns;

pub fn open_main(scores: &Vec<String>) {
    let patterns = get_patterns(scores, ".pdf");

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    println!("Opening {}...", path.display());
                    Command::new("open").arg(path).output().unwrap();
                }
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
