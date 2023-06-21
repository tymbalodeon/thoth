use crate::add_value_to_string_if_some;
use crate::commands::edit::edit_main;
use crate::commands::templates::form::get_form_templates;
use crate::commands::templates::lead::get_lead_templates;
use crate::commands::templates::piano::get_piano_template;
use crate::commands::templates::single::get_single_template;
use crate::commands::templates::{
    Template,
    Template::{Form, Lead, Piano, Single},
    TemplateFile,
};
use crate::config::Config;
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
) -> Vec<TemplateFile> {
    match template {
        Form => get_form_templates(title, subtitle, composer, arranger),
        Lead => {
            get_lead_templates(title, subtitle, composer, arranger, instrument)
        }
        Piano => get_piano_template(title, subtitle, composer, arranger),
        Single => get_single_template(
            title, subtitle, composer, arranger, instrument,
        ),
    }
}

fn create_file(
    template: TemplateFile,
    parent: &String,
    mut title: String,
) -> String {
    if let Some(filename) = template.filename {
        title = format!("{title}-{filename}.ily").as_mut().to_string();
    } else {
        title = format!("{title}.ly")
    }

    let filename = format!("{parent}/{title}");
    let path = Path::new(&filename);
    let file_display = path.display();

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(message) => panic!("couldn't create {file_display}: {message}"),
    };

    if let Err(message) = file.write_all(template.content.as_bytes()) {
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
        let file = create_file(template, &parent, file_system_title.clone());
        files.push(file)
    }

    if *edit {
        println!("Opening for editing...")
    }

    files
}

pub fn print_score_info(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &Option<String>,
    template: &Template,
) {
    let mut score_info = format!(
        "Created score for \"{title}\" using {:?} template:\n",
        template
    );

    score_info = add_value_to_string_if_some(score_info, "subtitle", subtitle);
    score_info.push_str(format!("Composer = {composer}\n").as_str());
    score_info = add_value_to_string_if_some(score_info, "Arranger", arranger);
    score_info =
        add_value_to_string_if_some(score_info, "Instrument", instrument);

    println!("{score_info}");
}

pub fn create_main(
    title: &String,
    subtitle: &Option<String>,
    composer: &Option<String>,
    arranger: &Option<String>,
    instrument: &Option<String>,
    template: &Option<Template>,
    edit: &bool,
) {
    let config = Config::from_config_file();

    let template = if let Some(template) = template {
        template
    } else {
        &config.template
    };

    let composer = if let Some(composer) = composer {
        composer
    } else {
        &config.composer
    };

    let files = create_score(
        title, subtitle, composer, arranger, instrument, template, edit,
    );

    print_score_info(
        title, subtitle, composer, arranger, instrument, template,
    );

    for file in files {
        println!("{file}");
    }

    if *edit {
        edit_main(&get_file_system_name(title));
    }
}
