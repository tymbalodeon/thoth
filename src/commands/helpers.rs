use std::fmt::{Display, Formatter, Result};

use bat::{PagingMode, PrettyPrinter};
use clap::ValueEnum;
use convert_case::{Case::Kebab, Casing};
use serde::{Deserialize, Serialize};

use super::{table::print_table, HelperCommand};

const HELPER_FILES: &[(&str, &[u8])] =
    &include!(concat!(env!("OUT_DIR"), "/helper_files.rs"));

#[derive(Clone, Debug, Deserialize, Serialize, ValueEnum)]
pub enum Helper {
    AddArticulations,
    AddFingerings,
    AddStringNumbers,
    BarNumbersCenter,
    BarNumbersLeft,
    CalculateGlissandoSlope,
    FakeClefBass,
    FakeClefTreble,
    HalfBracket,
    MoveArticulationsBelow,
    RemoveFingerings,
    SetBarsPerLine,
    Settings,
    WideTupletBrackets,
}

impl Display for Helper {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let display = format!("{self:?}").to_case(Kebab);
        write!(formatter, "{display}")
    }
}

fn display_helper_file(helper: &Helper) {
    for (file_path, _) in HELPER_FILES {
        let helper = helper.to_string();
        let file_name = format!("{helper}.ily");

        if file_path.ends_with(file_name.as_str()) {
            PrettyPrinter::new()
                .input_file(file_path)
                .language("lisp")
                .theme("gruvbox-dark")
                .line_numbers(true)
                .paging_mode(PagingMode::QuitIfOneScreen)
                .print()
                .unwrap();
        }
    }
}

pub fn main(command: &Option<HelperCommand>) {
    if command.is_some() {
        match command.as_ref().unwrap() {
            HelperCommand::Show { helper } => display_helper_file(helper),
        }

        return;
    }

    let titles = vec!["NAME".to_string(), "DESCRIPTION".to_string()];
    let rows = vec![
        ["add-articulations", "Add articulations"],
        ["add-fingerings", "Add fingerings"],
        ["add-string-numbers", "Add string numbers"],
        ["bar-numbers-center", "Center bar numbers"],
        [
            "bar-numbers-left",
            "Left algin bar numbers (will appear above the clef)",
        ],
        ["calculate-glissando-slope", "Custom glissando"],
        [
            "fake-clef-bass",
            "Display bass clef that doesn't affect the input notes",
        ],
        [
            "fake-clef-treble",
            "Display treble clef that doesn't affect the input notes",
        ],
        [
            "half-bracket",
            "Half bracket (e.g. for hand divisions in piano music)",
        ],
        ["move-articulations-below", "Move articulations below notes"],
        ["remove-fingerings", "Remove fingering engraver"],
        [
            "set-bars-per-line",
            "Specify the number of bars for the score, or for each line",
        ],
        ["settings", "Default global settings"],
        [
            "wide-tuplet-brackets",
            "Use wider-than-default tuplet brackets",
        ],
    ];

    let rows = rows
        .iter()
        .map(|row| row.iter().map(|value| value.to_string()).collect())
        .collect();

    print_table(titles, rows);
}

pub fn pushln(lines: &mut String, text: String) {
    lines.push_str(&text);
    lines.push('\n');
}
