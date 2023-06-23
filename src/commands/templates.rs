pub mod form;
pub mod lead;
pub mod piano;
pub mod single;

use std::process::Command;

use bat::{PagingMode, PrettyPrinter};
use clap::ValueEnum;
use indoc::formatdoc;
use regex::Regex;
use serde::{Deserialize, Serialize};

use self::form::get_form_templates;
use self::lead::get_lead_templates;
use self::piano::get_piano_template;
use self::single::get_single_template;
use super::{table::print_table, TemplateCommand};
use crate::{
    add_value_to_string_if_some, commands::create::get_file_system_name,
};

#[derive(Clone, Debug, Deserialize, Serialize, ValueEnum)]
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
    let mut header = formatdoc!(
        "
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

fn format_filename(filename: &Option<String>, title: &str) -> String {
    let filename = if let Some(filename) = filename {
        format!("{filename}.ily")
    } else {
        let title = get_file_system_name(title);
        format!("{title}.ly")
    };

    let lines = "-".repeat(filename.len());

    format!("% {lines}\n% {filename}\n% {lines}\n\n")
}

fn print_templates(templates: Vec<TemplateFile>, title: &str) {
    let mut lines: String = Default::default();

    for (index, template) in templates.iter().enumerate() {
        if index > 0 {
            lines.push('\n');
        }

        let filename = format_filename(&template.filename, title);
        lines.push_str(&filename);
        lines.push_str(&template.content);
        lines.push('\n');
    }

    PrettyPrinter::new()
        .input_from_bytes(lines.as_bytes())
        .colored_output(false)
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
        .unwrap();
}

fn show_template(template: &Template) {
    let title = &"Title".to_string();
    let subtitle = &Some("Subtitle".to_string());
    let composer = &"Composer".to_string();
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
            TemplateCommand::Show { template } => show_template(template),
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
