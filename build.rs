use std::{
    env,
    error::Error,
    fs::{read_dir, File},
    io::Write,
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_directory = env::var("OUT_DIR")?;
    let destination_path = Path::new(&out_directory).join("helper_files.rs");
    let mut helper_files = File::create(destination_path)?;

    writeln!(&mut helper_files, r##"["##,)?;

    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Faild to read CARGO_MANIFEST_DIR.");
    let source_directory = format!("{cargo_manifest_dir}/helpers");

    for file in read_dir(source_directory)? {
        let file = file?;

        if !file.file_type()?.is_file() {
            continue;
        }

        writeln!(
            &mut helper_files,
            r##"("{name}", include_bytes!(r#"{name}"#)),"##,
            name = file.path().display(),
        )?;
    }

    writeln!(&mut helper_files, r##"]"##,)?;

    Ok(())
}
