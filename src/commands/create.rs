use crate::config::get_scores_directory;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static PIANO_TEMPLATE: &str = "\\version \"2.24.0\"

\\include \"settings.ily\"
\\include \"style.ily\"

\\header {
  title = \"Title\"
  composer = \"Composer\"
}

key_and_time = {
  \\key c \\major
  \\time 4/4
}

upper_staff = \relative c'' {
  \\key_and_time
  | c1
}

lower_staff = \relative c {
  \\clef bass
  \\key_and_time
  | c1
}

\\layout {
  \\context {
    \\Score \\consists
    #(set-bars-per-line '(4))
  }
}

\\score {
  \\new PianoStaff \\with {
    instrumentName = \"Piano\"
  }
  <<
    \\new Staff = \"upper\" \\upper_staff
    \\new Staff = \"lower\" \\lower_staff
  >>
}
";

pub fn create_score(title: &String) {
    let scores_directory = get_scores_directory();
    let filename = format!("{scores_directory}/{title}.ly");
    let path = Path::new(&filename);
    let file_display = path.display();

    let mut file = match File::create(&path) {
        Err(message) => panic!("couldn't create {file_display}: {message}"),
        Ok(file) => file,
    };

    match file.write_all(PIANO_TEMPLATE.as_bytes()) {
        Err(message) => panic!("couldn't write to {file_display}: {message}"),
        Ok(_) => (),
    }
}
