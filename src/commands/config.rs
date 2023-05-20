use crate::config::Config;

pub fn display_config() {
    let config: Config = Config::new();

    let composer = config.composer;
    let scores_directory = config.scores_directory;
    let pdfs_directory = config.pdfs_directory;

    println!("Composer = {composer}");
    println!("Scores directory = {scores_directory}");
    println!("PDFs directory =  {pdfs_directory}");
}
