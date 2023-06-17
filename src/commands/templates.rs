use crate::add_value_to_string_if_some;
use regex::Regex;
use std::process::Command;

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

fn add_version_number(content: &str) -> String {
    let mut template = get_lilypond_version();
    template.push_str(content);
    template
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

pub fn get_piano_template(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
) -> String {
    let header = get_header(title, subtitle, composer, arranger);

    let content = format!(
        "\
\\include \"settings.ily\"

{header}

key_and_time = {{
  \\key c \\major
  \\time 4/4
}}

upper_staff = \\relative c'' {{
  \\key_and_time
  | c1
}}

lower_staff = \\relative c {{
  \\clef bass
  \\key_and_time
  | c1
}}

\\score {{
  \\new PianoStaff \\with {{
    instrumentName = \"Piano\"
  }}
  <<
    \\new Staff = \"upper\" \\upper_staff
    \\new Staff = \"lower\" \\lower_staff
  >>
}}"
    );

    add_version_number(&content)
}

pub fn get_single_template(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &String,
) -> String {
    let header = get_header(title, subtitle, composer, arranger);

    let content = format!(
        "\
\\include \"settings.ily\"

{header}

music = \\relative c'' {{
    \\key c \\major
    \\time 4/4
    | c1
}}

\\score {{
    \\new Staff \\with {{
        instrumentName = \"{instrument}\"
        \\numericTimeSignature
    }} {{
        \\compressMMRests
        \\music
    }}
}}"
    );

    add_version_number(&content)
}
