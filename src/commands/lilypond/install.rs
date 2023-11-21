use std::fs::File;
use std::io;

use crate::commands::{
    lilypond::{
        get_tag_names, get_versions, global::read_global_version,
        is_valid_version, list_remote::filter_versions,
    },
    VersionStability,
};

use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;
use shellexpand::tilde;

#[derive(Deserialize)]
struct DirectAssetUrl {
    direct_asset_url: String,
}

#[derive(Deserialize)]
struct Links {
    links: Vec<DirectAssetUrl>,
}

#[derive(Deserialize)]
struct Response {
    assets: Links,
}

fn get_latest_version_by_stability(stability: VersionStability) -> String {
    let versions = get_versions();

    filter_versions(&versions, stability)
        .first()
        .unwrap()
        .to_string()
}

fn get_latest_version(version: &str) -> Option<String> {
    match version {
        "latest-stable" => {
            Some(get_latest_version_by_stability(VersionStability::Stable))
        }
        "latest-unstable" => {
            Some(get_latest_version_by_stability(VersionStability::Unstable))
        }
        _ => None,
    }
}

fn parse_version(version: &str) -> String {
    let latest_version = get_latest_version(version);

    if let Some(version) = latest_version {
        version
    } else {
        version.to_string()
    }
}

fn get_direct_asset_url(version: &str) -> Option<String> {
    let version_regex = parse_version(version);
    let re = Regex::new(&version_regex).unwrap();
    let tag_name = get_tag_names()
        .iter()
        .find(|tag_name| re.is_match(tag_name))
        .map(|tag_name| tag_name.to_string())
        .unwrap()
        .replace("release/", "release%2F");
    let url = format!(
        "https://gitlab.com/api/v4/projects/18695663/releases/{tag_name}"
    );

    get(url)
        .unwrap()
        .json::<Response>()
        .unwrap()
        .assets
        .links
        .iter()
        .find(|url| url.direct_asset_url.contains("darwin"))
        .map(|url| url.direct_asset_url.to_string())
}

pub fn install(version: &Option<String>) {
    let value = if let Some(value) = version {
        value.to_string()
    } else {
        read_global_version()
    };

    if !is_valid_version(&value) {
        println!("invalid version specifier");
        return;
    }

    if let Some(direct_asset_url) = get_direct_asset_url(&value) {
        println!("Downloading from {direct_asset_url}...");
        let content = get(direct_asset_url).unwrap().bytes().unwrap();
        let mut output =
            File::create(tilde("~/Desktop/testing.tar.gz").to_string())
                .unwrap();
        io::copy(&mut content.as_ref(), &mut output).unwrap();
    } else {
        println!("No assets found.")
    }
}
