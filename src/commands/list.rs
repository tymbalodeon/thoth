use crate::commands::patterns::get_patterns;
use glob::glob;

pub fn list_pdfs(scores: &Vec<String>) {
    let patterns = get_patterns(scores, ".pdf");

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{}", path.display()),
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
