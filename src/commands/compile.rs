use glob::glob;

use crate::config::get_scores_directory;

pub fn compile_pdfs(scores: &Vec<String>) {
    let scores_directory = get_scores_directory();
    let base = format!("{scores_directory}/**/");
    let extension = ".pdf";

    let patterns: Vec<String> = if scores.len() > 0 {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    };

    for pattern in patterns {
        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("Compiled {}", path.display()),
                Err(message) => println!("{:?}", message),
            }
        }
    }
}
