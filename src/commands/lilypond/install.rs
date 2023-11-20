use crate::commands::lilypond::{
    global::read_global_version, is_valid_version,
};

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

fn get_direct_asset_url(version: &String) -> String {
    get(format!(
        "https://gitlab.com/api/v4/projects/18695663/releases/v{version}"
    ))
    .unwrap()
    .json::<Response>()
    .unwrap()
    .assets
    .links
    .iter()
    .filter(|url| url.direct_asset_url.contains("darwin"))
    .map(|url| url.direct_asset_url.to_string())
    .next()
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
