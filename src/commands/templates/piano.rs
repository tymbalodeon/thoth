use crate::commands::templates::{
    get_header, get_lilypond_version, TemplateFile,
};

pub fn get_piano_template(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
) -> Vec<TemplateFile> {
    let lilypond_version = get_lilypond_version();
    let header = get_header(title, subtitle, composer, arranger);

    let content = format!(
        "\
\\version {lilypond_version}

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

    vec![TemplateFile {
        filename: None,
        content,
    }]
}
