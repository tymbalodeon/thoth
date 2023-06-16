use super::templates::{get_piano_template, get_single_template};
use crate::config::Config;
use crate::Template;
use crate::Template::{Form, Lead, Piano, Single};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

fn get_templates(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &String,
    template: &Template,
) -> Vec<String> {
    match template {
        Form => vec![get_piano_template(title, subtitle, composer, arranger)],
        Lead => vec![get_piano_template(title, subtitle, composer, arranger)],
        Piano => vec![get_piano_template(title, subtitle, composer, arranger)],
        Single => vec![get_single_template(
            title, subtitle, composer, arranger, instrument,
        )],
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

fn get_file_system_name(text: &str) -> String {
    text.replace(' ', "-").to_lowercase()
}

pub fn create_score(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &Option<String>,
    template: &Template,
    edit: &bool,
) -> Vec<String> {
    let config = Config::from_config_file();
    let scores_directory = config.scores_directory;
    let composer_directory = get_file_system_name(composer);
    let file_system_title = get_file_system_name(title);
    let parent = format!(
        "{scores_directory}/scores/{composer_directory}/{file_system_title}"
    );

    create_dir_all(&parent).unwrap();

    let instrument = if let Some(instrument) = instrument {
        instrument
    } else {
        &config.instrument
    };

    let templates = get_templates(
        title, subtitle, composer, arranger, instrument, template,
    );
    let mut files = Vec::new();

    for template in templates {
        let file = create_file(template, &parent, &file_system_title);
        files.push(file)
    }

    if *edit {
        println!("Opening for editing...")
    }

    files
}
