use super::create::{create_main, ScoreFileSettings};

pub fn sketch_main() {
    create_main(ScoreFileSettings::default(), &true, &true, &None, &None);
}
