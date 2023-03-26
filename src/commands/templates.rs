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

    format!("\\version {found}\n\n")
}

fn add_version_number(content: &str) -> String {
    let mut template = get_lilypond_version();
    template.push_str(content);
    template
}

pub fn get_piano_template(title: &String, composer: &String) -> String {
    let content = format!(
        "\
\\include \"settings.ily\"
\\include \"style.ily\"

\\header {{
  title = \"{title}\"
  composer = \"{composer}\"
}}

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

\\layout {{
  \\context {{
    \\Score \\consists
    #(set-bars-per-line '(4))
  }}
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

pub fn get_single_template(title: &String, composer: &String) -> String {
    let content = format!(
        "\
\\include \"settings.ily\"

\\header {{
    title = \"{title}\"
    composer = \"{composer}\"
}}

music = \\relative c'' {{
    \\key c \\major
    \\time 4/4
    | c1
}}

\\score {{
    \\new Staff \\with {{
        instrumentName = \"Instrument\"
        \\numericTimeSignature
    }} {{
        \\compressMMRests
        \\music
    }}
}}"
    );

    add_version_number(&content)
}
