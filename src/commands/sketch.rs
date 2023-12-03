use super::create::{self, ScoreFileSettings};

pub fn main(lilypond_version: &Option<String>) {
    create::main(
        &ScoreFileSettings::default(),
        true,
        true,
        lilypond_version,
        &None,
        &None,
    );
}
