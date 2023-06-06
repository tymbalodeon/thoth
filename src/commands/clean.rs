use crate::commands::patterns::get_patterns;
use glob::glob;
use std::fs::remove_file;
use std::io::Write;
use std::io::{stdin, stdout};

fn confirmation_received() -> bool {
    print!("Are you sure you want to remove all pdfs? [y/n] ");
    stdout().flush().unwrap();
    let mut response = String::new();
    let stdin = stdin();
    stdin
        .read_line(&mut response)
        .expect("Failed to read input.");
    response = response.replace('\n', "");

    response.eq("y") || response.eq("Y")
}

pub fn clean_pdfs(scores: &Vec<String>) {
    if scores.is_empty() && !confirmation_received() {
        return;
    };

    let patterns = get_patterns(scores, ".pdf");

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if let Err(error) = remove_file(&path) {
                        println!("Removed {}", path.display());
                        println!(
                            "Failed to remove {} ({error})",
                            path.display()
                        );
                    } else {
                        println!("Removed {}", path.display());
                    };
                }
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
