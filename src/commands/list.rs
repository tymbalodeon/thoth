use glob::glob;
use titlecase::titlecase;

use super::compile::is_compiled;
use super::get_pdfs_directory_from_arg;
use crate::commands::scores::get_found_scores;
use crate::commands::table::print_table;

struct Composition {
    artist: String,
    title: String,
    is_compiled: bool,
}

fn get_display(value: &String) -> String {
    titlecase(&value.replace('-', " "))
}

impl Composition {
    fn get_row_values(&self) -> Vec<String> {
        let artist = get_display(&self.artist);
        let title = get_display(&self.title);
        let pdf = self.is_compiled.to_string();

        vec![artist, title, pdf]
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

pub fn list_main(
    search_terms: &Vec<String>,
    outdated: &bool,
    compiled: &bool,
    search_artist: &bool,
    search_title: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let mut compositions = vec![];

    let found_scores = get_found_scores(
        search_terms,
        search_artist,
        search_title,
        scores_directory,
    );

    for score in found_scores {
        let mut pdf = false;
        let path = String::from(score.file_name().unwrap().to_str().unwrap());
        let pdfs_directory = get_pdfs_directory_from_arg(pdfs_directory);
        let pattern = format!("{pdfs_directory}/{}*.pdf", path);

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
            *outdated && !pdf || *compiled && pdf || !*outdated && !*compiled;

        if should_display {
            let artist = score
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let pattern = format!("{}/*.ly", score.display());
            let mut title = "".to_string();

            for ly_file in glob(&pattern)
                .expect("Failed to read glob pattern")
                .flatten()
            {
                title =
                    ly_file.file_stem().unwrap().to_str().unwrap().to_string();
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
            "ARTIST".to_string(),
            "TITLE".to_string(),
            "STATUS".to_string(),
        ];

        let rows = compositions
            .iter()
            .map(|composition| composition.get_row_values())
            .collect();

        print_table(header, rows);
    }
}
