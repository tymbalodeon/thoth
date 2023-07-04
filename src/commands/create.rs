use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

use super::add_value_to_string_if_some;
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
use crate::commands::{
    get_composer_from_arg, get_scores_directory_from_arg,
    get_template_from_arg,
};
use crate::config::Config;

pub struct ScoreFileSettings {
    pub title: String,
    pub subtitle: Option<String>,
    pub composer: Option<String>,
    pub arranger: Option<String>,
    pub template: Option<Template>,
    pub instrument: Option<String>,
}

impl Default for ScoreFileSettings {
    fn default() -> Self {
        ScoreFileSettings {
            title: "Sketch".to_string(),
            subtitle: None,
            composer: None,
            arranger: None,
            instrument: None,
            template: Some(Template::Piano),
        }
    }
}

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

pub fn create_file(
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

pub fn get_file_system_name(text: &str) -> String {
    text.replace(' ', "-").to_lowercase()
}

pub fn create_score(
    settings: &ScoreFileSettings,
    edit: &bool,
    scores_directory: &Option<String>,
    is_sketch: &bool,
) -> Vec<String> {
    let title = &settings.title;
    let subtitle = &settings.subtitle;
    let composer = get_composer_from_arg(&settings.composer);
    let arranger = &settings.arranger;
    let template = get_template_from_arg(&settings.template);
    let instrument = &settings.instrument;
    let file_system_title = get_file_system_name(title);

    let parent = if *is_sketch {
        format!("/tmp/{file_system_title}")
    } else {
        let scores_directory = get_scores_directory_from_arg(scores_directory);
        let composer_directory = get_file_system_name(&composer);
        format!("{scores_directory}/scores/{composer_directory}/{file_system_title}")
    };

    create_dir_all(&parent).unwrap();
    let config = Config::from_config_file();

    let instrument = if let Some(instrument) = instrument {
        instrument
    } else {
        &config.instrument
    };

    let templates = get_templates(
        title, subtitle, &composer, arranger, instrument, &template,
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
    settings: ScoreFileSettings,
    edit: &bool,
    is_sketch: &bool,
    scores_directory: &Option<String>,
    pdfs_directory: &Option<String>,
) {
    let title = &settings.title;
    let subtitle = &settings.subtitle;
    let composer = &settings.composer;
    let arranger = &settings.arranger;
    let template = &settings.template;
    let instrument = &settings.instrument;

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

    let files = create_score(&settings, edit, scores_directory, is_sketch);

    print_score_info(
        title, subtitle, composer, arranger, instrument, template,
    );

    for file in files {
        println!("{file}");
    }

    if *edit {
        edit_main(
            &get_file_system_name(title),
            &false,
            &false,
            &true,
            scores_directory,
            pdfs_directory,
        );
    }
}
