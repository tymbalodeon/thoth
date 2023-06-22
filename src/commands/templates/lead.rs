use crate::commands::templates::{
    get_header, get_lilypond_version, TemplateFile,
};
use indoc::formatdoc;

fn get_lead_changes() -> String {
    formatdoc!(
        "
        verse = \\chordmode {{
          | c1
        }}

        chorus = \\chordmode {{
          | c1
        }}

        changes = \\chords {{
          \\verse
          \\chorus
        }}
"
    )
}

fn get_lead_lyrics() -> String {
    formatdoc!(
        "
        verse = \\lyricmode {{
          Verse one
        }}

        chorus = \\lyricmode {{
          Chorus
        }}

        words = \\lyricmode {{
          \\verse
          \\chorus
        }}
"
    )
}

fn get_lead_main(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &String,
) -> String {
    let lilypond_version = get_lilypond_version();
    let header = get_header(title, subtitle, composer, arranger);

    formatdoc!(
        "
        {lilypond_version}

        \\include \"helpers/settings.ily\"

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
        }}
"
    )
}

fn get_lead_melody() -> String {
    formatdoc!(
        "
        verse = \\relative c'' {{
          | c1
        }}

        chorus = \\relative c'' {{
          | c1
        }}

        melody = {{
          \\verse
          \\chorus
        }}
"
    )
}

fn get_lead_structure() -> String {
    formatdoc!(
        "
        key_and_time = {{
          \\key c \\major
          \\time 4/4
        }}

        verse = {{
          | s1 * 4
        }}

        chorus = {{
          | s1 * 4
        }}

        structure = {{
          \\key_and_time
          \\verse
          \\chorus
        }}
"
    )
}

pub fn get_lead_templates(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &String,
) -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            filename: None,
            content: get_lead_main(
                title, subtitle, composer, arranger, instrument,
            ),
        },
        TemplateFile {
            filename: Some("changes".to_string()),
            content: get_lead_changes(),
        },
        TemplateFile {
            filename: Some("lyrics".to_string()),
            content: get_lead_lyrics(),
        },
        TemplateFile {
            filename: Some("melody".to_string()),
            content: get_lead_melody(),
        },
        TemplateFile {
            filename: Some("structure".to_string()),
            content: get_lead_structure(),
        },
    ]
}
