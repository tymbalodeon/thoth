use crate::config::Config;

pub fn get_scores_directory_pattern() -> String {
    let config: Config = Config::new();
    let scores_directory = config.scores_directory();
    format!("{scores_directory}/**/")
}

pub fn get_patterns(scores: &Vec<String>, extension: &str) -> Vec<String> {
    let base = get_scores_directory_pattern();

    if scores.len() > 0 {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    }
}
