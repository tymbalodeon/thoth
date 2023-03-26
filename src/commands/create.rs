use super::templates::{get_piano_template, get_single_template};
use crate::config::get_scores_directory;
use crate::Template;
use crate::Template::{Form, Lead, Piano, Single};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn get_template(template: &Template, composer: &String, title: &String) -> String {
    match template {
        Form => get_piano_template(title, composer),
        Lead => get_piano_template(title, composer),
        Piano => get_piano_template(title, composer),
        Single => get_single_template(title, composer),
    }
}

pub fn create_score(template: &Template, composer: &String, title: &String) -> String {
    let scores_directory = get_scores_directory();
    let composer_directory = composer.replace(' ', "-").to_lowercase();
    let parent_directory_name = format!("{scores_directory}/scores/{composer_directory}/{title}");
    create_dir_all(&parent_directory_name).unwrap();
    let filename = format!("{parent_directory_name}/{title}.ly");
    let path = Path::new(&filename);
    let file_display = path.display();

    let mut file = match File::create(path) {
        Err(message) => panic!("couldn't create {file_display}: {message}"),
        Ok(file) => file,
    };

    let template = get_template(template, composer, title);

    if let Err(message) = file.write_all(template.as_bytes()) {
        panic!("couldn't write to {file_display}: {message}")
    };

    filename
}
