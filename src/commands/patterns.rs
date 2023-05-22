use crate::config::Config;

pub fn get_patterns(scores: &Vec<String>, extension: &str) -> Vec<String> {
    let config: Config = Config::new();
    let scores_directory = config.scores_directory();
    let base = format!("{scores_directory}/**/");

    if scores.len() > 0 {
        scores
            .iter()
            .map(|score| format!("{base}*{score}*{extension}"))
            .collect()
    } else {
        vec![format!("{base}*{extension}")]
    }
}
