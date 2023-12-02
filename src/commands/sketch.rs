use super::create::{self, ScoreFileSettings};

pub fn main() {
    create::main(&ScoreFileSettings::default(), true, true, &None, &None);
}
