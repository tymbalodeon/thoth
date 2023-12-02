use glob::glob;
use owo_colors::OwoColorize;
use titlecase::titlecase;

use super::compile::is_compiled;
use super::get_pdfs_directory_from_arg;
use crate::commands::scores::search;
use crate::commands::table;

struct Composition {
    artist: String,
    title: String,
    is_compiled: bool,
}

fn get_display(value: &str) -> String {
    titlecase(&value.replace('-', " "))
}

impl Composition {
    fn get_row_values(&self) -> Vec<String> {
        let artist = get_display(&self.artist);
        let title = get_display(&self.title);
        let pdf = if self.is_compiled {
            "compiled".green().to_string()
        } else {
            "missing".red().to_string()
        };

        vec![artist.yellow().to_string(), title.bold().to_string(), pdf]
    }
}

fn remove_leading_article(value: &String, article: &str) -> String {
    let article = format!("{article}-");

    if value.to_lowercase().starts_with(&article) {
        value[article.len()..].to_string()
    } else {
        value.to_string()
    }
}

fn remove_leading_articles(mut value: String) -> String {
    value = remove_leading_article(&value, "the");
    value = remove_leading_article(&value, "a");
    value = remove_leading_article(&value, "an");

    value
}

pub fn main(
    search_terms: &Vec<String>,
    outdated: bool,
    compiled: bool,
    search_artist: bool,
    search_title: bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let mut compositions = vec![];

    let found_scores =
        search(search_terms, search_artist, search_title, scores_directory);

    for score in found_scores {
        let mut pdf = false;
        let err = "Failed to parse score file name.";
        let path =
            String::from(score.file_name().expect(err).to_str().expect(err));
        let pdfs_directory = get_pdfs_directory_from_arg(pdfs_directory);
        let pattern = format!("{pdfs_directory}/{path}*.pdf");

        for pdf_file in glob(&pattern)
            .expect("Failed to read glob pattern")
            .flatten()
        {
            if is_compiled(&score, &pdf_file) {
                pdf = true;
                break;
            }
        }

        let should_display =
            outdated && !pdf || compiled && pdf || !outdated && !compiled;

        if should_display {
            let err = "Failed to get score artist.";

            let artist = score
                .parent()
                .expect(err)
                .file_name()
                .expect(err)
                .to_str()
                .expect(err)
                .to_string();

            let pattern = format!("{}/*.ly", score.display());
            let mut title = String::new();

            for ly_file in glob(&pattern)
                .expect("Failed to read glob pattern")
                .flatten()
            {
                let err = "Failed to get score title.";

                title = ly_file
                    .file_stem()
                    .expect(err)
                    .to_str()
                    .expect(err)
                    .to_string();
            }

            compositions.push(Composition {
                artist,
                title,
                is_compiled: pdf,
            });
        }
    }

    if !compositions.is_empty() {
        compositions.sort_by(|a, b| {
            let self_artist = remove_leading_articles(a.artist.clone());
            let other_artist = remove_leading_articles(b.artist.clone());
            let self_title = remove_leading_articles(a.title.clone());
            let other_title = remove_leading_articles(b.title.clone());

            self_artist
                .cmp(&other_artist)
                .then(self_title.cmp(&other_title))
        });

        let header = vec![
            "Artist".italic().to_string(),
            "Title".italic().to_string(),
            "Status".italic().to_string(),
        ];

        let rows = compositions
            .iter()
            .map(Composition::get_row_values)
            .collect();

        table::print(&header, rows);
    }
}
