use std::borrow::Cow;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

pub fn print_contents(directory: Cow<str>) {
    println!("Default scores directory: {directory}");
    let directory_walker = WalkDir::new(directory.as_ref()).into_iter();

    for entry in directory_walker.filter_entry(|entry| !is_hidden(entry)) {
        println!("\t{}", entry.unwrap().path().display());
    }
}
