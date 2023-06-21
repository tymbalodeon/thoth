use crate::commands::templates::{
    get_header, get_lilypond_version, TemplateFile,
};

fn get_form_changes() -> String {
    "\
changes_verse = \\chords {
  | c1
}

changes_chorus = \\chords {
  | c1
}
"
    .to_string()
}

fn get_form_main(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
) -> String {
    let lilypond_version = get_lilypond_version();
    let header = get_header(title, subtitle, composer, arranger);

    format!(
        "\
{lilypond_version}

\\include \"settings.ily\"
\\include \"helpers/set-bars-per-line.ily\"
\\include \"melody.ily\"
\\include \"changes.ily\"
\\include \"structure.ily\"

\\paper {{
  score-system-spacing.basic-distance = #18
}}

{header}

\\layout {{
  ragged-last = ##f
  \\context {{
    \\Score \\consists
    #(set-bars-per-line '(4))
  }}
}}

\\markup \\vspace #1

\\score {{
  <<
    \\numericTimeSignature
    \\changes_verse
    \\new Staff \\with {{
      instrumentName = \\markup \\box \"Verse\"
    }} {{
      <<
        \\melody_verse
        \\structure_verse
      >>
    }}
  >>
}}

\\score {{
  <<
    \\numericTimeSignature
    \\changes_chorus
    \\new Staff \\with {{
      instrumentName = \\markup \\box \"Chorus\"
    }} {{
      <<
        \\melody_chorus
        \\structure_chorus
      >>
    }}
  >>
}}

\\markup \\vspace #2

\\markup \\fill-line {{
  \\column
  \\override #'(padding . 5)
  \\table #'(1 -1 -1)
  {{
    \\bold Verse \\italic \"\" \"\"
    \\bold Chorus \\italic \"\" \"\"
  }}
}}
"
    )
}

fn get_form_melody() -> String {
    "\
melody_verse = \\new Voice \\with {
  \\consists \"Pitch_squash_engraver\"
} \\relative c' {
  \\improvisationOn
  | c1
}

melody_chorus = \\new Voice \\with {
  \\consists \"Pitch_squash_engraver\"
} \\relative c' {
  \\improvisationOn
  | c1
}
"
    .to_string()
}

fn get_form_structure() -> String {
    "\
key_and_time = {
  \\key c \\major
  \time 4/4
}

structure_verse = {
  \\key_and_time
  | s1 * 4
  \\bar \"||\"
}

structure_chorus = {
  \\key_and_time
  | s1 * 4
  \\bar \"||\"
}
"
    .to_string()
}

pub fn get_form_templates(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
) -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            filename: None,
            content: get_form_main(title, subtitle, composer, arranger),
        },
        TemplateFile {
            filename: Some("changes".to_string()),
            content: get_form_changes(),
        },
        TemplateFile {
            filename: Some("melody".to_string()),
            content: get_form_melody(),
        },
        TemplateFile {
            filename: Some("structure".to_string()),
            content: get_form_structure(),
        },
    ]
}
