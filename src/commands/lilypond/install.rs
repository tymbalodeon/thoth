use std::fs::{create_dir_all, remove_file, File};
use std::io::{self, copy};
use std::path::Path;

use flate2::read::GzDecoder;
use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;
use shellexpand::tilde;
use tar::Archive;

use super::global::get_global_version;
use super::GITLAB_URL;
use crate::commands::{
    lilypond::{
        filter_versions, get_tag_names, get_versions,
        global::read_global_version, is_valid_version, INSTALL_PATH,
    },
    VersionStability,
};

#[derive(Deserialize)]
struct AssetLink {
    direct_asset_url: String,
    name: String,
}

fn get_latest_version_by_stability(stability: &VersionStability) -> String {
    let versions = get_versions();

    (*filter_versions(&versions, stability)
        .first()
        .expect("Failed to get latest version."))
    .to_string()
}

pub fn get_dynamic_version(version: &str) -> Option<String> {
    match version {
        "latest-stable" => {
            Some(get_latest_version_by_stability(&VersionStability::Stable))
        }
        "latest-unstable" => {
            Some(get_latest_version_by_stability(&VersionStability::Unstable))
        }

        "global" => Some(get_global_version()),
        _ => None,
    }
}

pub fn parse_version(version: &str) -> String {
    get_dynamic_version(version)
        .map_or_else(|| version.to_string(), |version| version)
}

fn get_asset_link(version: &str) -> Option<AssetLink> {
    let version_regex = parse_version(version);
    let re = Regex::new(&version_regex)
        .expect("Failed to parse lilypond version regex.");
    let tag_name = get_tag_names()
        .iter()
        .find(|tag_name| re.is_match(tag_name))
        .map(ToString::to_string)
        .expect("Failed to get lilypond release tag name.")
        .replace("release/", "release%2F");
    let url = format!("{GITLAB_URL}/{tag_name}/assets/links");
    let err = "Failed to GET lilypond version asset link from GitLab.";

    get(url)
        .expect(err)
        .json::<Vec<AssetLink>>()
        .expect(err)
        .iter()
        .find(|link| link.direct_asset_url.contains("darwin"))
        .map(|link| AssetLink {
            direct_asset_url: link.direct_asset_url.to_string(),
            name: link.name.to_string(),
        })
}

pub fn get_install_path() -> String {
    tilde(INSTALL_PATH).to_string()
}

fn download_asset(asset_link: AssetLink) {
    let install_path = get_install_path();
    let file_path = format!("{}/{}", install_path, asset_link.name);
    let err = "Failed to get version path.";

    let version_path = format!(
        "{install_path}/{}",
        &file_path
            .split("thoth/")
            .last()
            .expect(err)
            .split("-darwin")
            .next()
            .expect(err)
    );

    if Path::new(&version_path).exists() {
        return;
    }

    create_dir_all(&install_path)
        .expect("Failed to create lilypond installations folder. ");

    println!("Downloading {}...", asset_link.direct_asset_url);

    let err = "Failed to download lilypond.";
    let content = get(asset_link.direct_asset_url)
        .expect(err)
        .bytes()
        .expect(err);
    let mut file = File::create(tilde(&file_path).to_string())
        .expect("Failed to create download file.");

    copy(&mut content.as_ref(), &mut file)
        .expect("Failed to save contents to file.");

    let mut archive = Archive::new(GzDecoder::new(
        File::open(&file_path).expect("Failed to read lilypond zip."),
    ));

    println!("Unpacking {}...", asset_link.name);

    archive
        .unpack(install_path)
        .expect("Failed to lilypond zip.");
    remove_file(file_path).expect("Failed to remove file.");
}

pub fn install(version: &Option<String>) -> io::Result<()> {
    let value = if let Some(value) = version {
        value.to_string()
    } else {
        read_global_version()?
    };

    if !is_valid_version(&value) {
        println!("invalid version specifier");

        return Ok(());
    }

    get_asset_link(&value).map_or_else(
        || {
            println!("No assets found.");
        },
        |asset_link| {
            download_asset(asset_link);
        },
    );

    Ok(())
}
