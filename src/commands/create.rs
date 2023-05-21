use super::templates::{get_piano_template, get_single_template};
use crate::config::Config;
use crate::Template;
use crate::Template::{Form, Lead, Piano, Single};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn get_templates(
    template: &Template,
    composer: &String,
    title: &String,
) -> Vec<String> {
    match template {
        Form => vec![get_piano_template(title, composer)],
        Lead => vec![get_piano_template(title, composer)],
        Piano => vec![get_piano_template(title, composer)],
        Single => vec![get_single_template(title, composer)],
    }
}

fn create_file(template: String, parent: &String, title: &String) -> String {
    let filename = format!("{parent}/{title}.ly");
    let path = Path::new(&filename);
    let file_display = path.display();

    let mut file = match File::create(path) {
        Err(message) => panic!("couldn't create {file_display}: {message}"),
        Ok(file) => file,
    };

    if let Err(message) = file.write_all(template.as_bytes()) {
        panic!("couldn't write to {file_display}: {message}")
    };

    filename
}

pub fn create_score(
    template: &Template,
    composer: &String,
    title: &String,
) -> Vec<String> {
    let scores_directory = Config::new().scores_directory();
    let composer_directory = composer.replace(' ', "-").to_lowercase();
    let parent =
        format!("{scores_directory}/scores/{composer_directory}/{title}");
    create_dir_all(&parent).unwrap();
    let templates = get_templates(template, composer, title);
    let mut filenames = Vec::new();

    for template in templates {
        let filename = create_file(template, &parent, title);
        filenames.push(filename)
    }

    filenames
}
