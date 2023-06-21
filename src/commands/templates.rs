use crate::add_value_to_string_if_some;
use clap::ValueEnum;
use prettytable::{format, Cell, Row, Table};
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

    format!("\\version \"{found}\"\n\n")
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
    header.push_str(format!("composer = \"{composer}\"\n").as_str());
    header = add_value_to_string_if_some(header, "arranger", arranger);
    header.push_str("}\n");

    header
}

pub fn templates_main() {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["NAME", "DESCRIPTION"]);

    let values = vec![
        [
            "Form",
            "Form chart with separate sections and form summary at the bottom",
        ],
        ["Lead", "Lead sheet showing melody and chords"],
        ["Piano", "Piano staff score"],
        ["Single", "Score for a single staff instrument"],
    ];

    for value in values {
        let cells: Vec<Cell> =
            value.iter().map(|item| Cell::new(item)).collect();
        table.add_row(Row::new(cells));
    }

    println!();
    table.printstd();
}
