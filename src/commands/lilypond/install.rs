use crate::commands::lilypond::{
    get_tag_names, global::read_global_version, is_valid_version,
};

use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;

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

fn get_direct_asset_url(version_regex: &str) -> String {
    let re = Regex::new(version_regex).unwrap();
    let tag_name = get_tag_names()
        .iter()
        .find(|tag_name| re.is_match(tag_name))
        .map(|tag_name| tag_name.to_string())
        .unwrap();
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
        .unwrap()
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

    let direct_asset_url = get_direct_asset_url(&value);

    println!("Downloading from {direct_asset_url}...");
}
