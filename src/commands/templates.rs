use self::form::get_form_templates;
use self::lead::get_lead_templates;
use self::piano::get_piano_template;
use self::single::get_single_template;
use super::{table::print_table, TemplateCommand};
use crate::{
    add_value_to_string_if_some, commands::create::get_file_system_name,
};
use clap::ValueEnum;
use regex::Regex;
use serde::Deserialize;
use std::process::Command;
pub mod form;
pub mod lead;
pub mod piano;
pub mod single;

#[derive(Clone, Debug, Deserialize, ValueEnum)]
pub enum Template {
    Form,
    Lead,
    Piano,
    Single,
}

#[derive(Debug)]
pub struct TemplateFile {
    pub filename: Option<String>,
    pub content: String,
}

fn get_lilypond_version() -> String {
    let output = String::from_utf8(
        Command::new("lilypond")
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let pattern = Regex::new(r"\d\.\d{2}\.\d").unwrap();

    let found = pattern
        .captures_iter(&output)
        .next()
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_owned();

    format!("\\version \"{found}\"")
}

fn get_header(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
) -> String {
    let mut header = format!(
        "\
\\header {{
  title = \"{title}\"
"
    );

    header = add_value_to_string_if_some(header, "subtitle", subtitle);
    header.push_str(format!("  composer = \"{composer}\"\n").as_str());
    header = add_value_to_string_if_some(header, "arranger", arranger);
    header.push('}');

    header
}

fn print_templates(templates: Vec<TemplateFile>, title: &str) {
    for (index, template) in templates.iter().enumerate() {
        if index > 0 {
            println!();
        }

        let filename = if let Some(filename) = &template.filename {
            format!("{filename}.ily")
        } else {
            let title = get_file_system_name(title);
            format!("{title}.ly")
        };

        let lines = "-".repeat(filename.len());
        println!("% {lines}\n% {filename}\n% {lines}\n");
        println!("{}", template.content);
    }
}

fn show_template(template: &Template) {
    let title = &"Title".to_string();
    let subtitle = &Some("Subtitle".to_string());
    let composer = &"Compsoer".to_string();
    let arranger = &Some("Arranger".to_string());
    let instrument = &"Instrument".to_string();

    match template {
        Template::Form => {
            let templates =
                get_form_templates(title, subtitle, composer, arranger);
            print_templates(templates, title);
        }
        Template::Lead => {
            let templates = get_lead_templates(
                title, subtitle, composer, arranger, instrument,
            );
            print_templates(templates, title);
        }
        Template::Piano => {
            let templates =
                get_piano_template(title, subtitle, composer, arranger);
            print_templates(templates, title);
        }
        Template::Single => {
            let templates = get_single_template(
                title, subtitle, composer, arranger, instrument,
            );
            print_templates(templates, title);
        }
    }
}

pub fn templates_main(command: &Option<TemplateCommand>) {
    if command.is_some() {
        match command.as_ref().unwrap() {
            TemplateCommand::Show { template } => match template {
                Template::Form => show_template(&Template::Form),
                Template::Lead => show_template(&Template::Lead),
                Template::Piano => show_template(&Template::Piano),
                Template::Single => show_template(&Template::Single),
            },
        }

        return;
    }

    let titles = vec!["NAME".to_string(), "DESCRIPTION".to_string()];
    let rows = vec![
        [
            "Form",
            "Form chart with separate sections and form summary at the bottom",
        ],
        ["Lead", "Lead sheet showing melody and chords"],
        ["Piano", "Piano staff score"],
        ["Single", "Score for a single staff instrument"],
    ];

    let rows = rows
        .iter()
        .map(|row| row.iter().map(|value| value.to_string()).collect())
        .collect();

    print_table(titles, rows);
}
