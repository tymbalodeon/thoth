use crate::commands::templates::{
    get_header, get_lilypond_version, TemplateFile,
};

pub fn get_single_template(
    title: &String,
    subtitle: &Option<String>,
    composer: &String,
    arranger: &Option<String>,
    instrument: &String,
) -> Vec<TemplateFile> {
    let lilypond_version = get_lilypond_version();
    let header = get_header(title, subtitle, composer, arranger);

    let content = format!(
        "\
\\version {lilypond_version}

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

    vec![TemplateFile {
        filename: None,
        content,
    }]
}
