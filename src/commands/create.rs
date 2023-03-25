use super::templates::{get_form_template, get_piano_template, get_single_template};
use crate::config::get_scores_directory;
use crate::Template;
use crate::Template::{Form, Lead, Piano, Single};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn get_template(template: &Template, composer: &String, title: &String) -> String {
    let get_template = match template {
        Form => get_piano_template,
        Lead => get_form_template,
        Piano => get_piano_template,
        Single => get_single_template,
    };

    get_template(title, composer)
}

pub fn create_score(template: &Template, composer: &String, title: &String) {
    let scores_directory = get_scores_directory();
    let filename = format!("{scores_directory}/{title}.ly");
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
}
