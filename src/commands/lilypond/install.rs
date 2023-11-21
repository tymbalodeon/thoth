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
struct AssetLink {
    direct_asset_url: String,
    name: String,
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

fn get_asset_link(version: &str) -> Option<AssetLink> {
    let version_regex = parse_version(version);
    let re = Regex::new(&version_regex).unwrap();
    let tag_name = get_tag_names()
        .iter()
        .find(|tag_name| re.is_match(tag_name))
        .map(|tag_name| tag_name.to_string())
        .unwrap()
        .replace("release/", "release%2F");
    let url = format!(
        "https://gitlab.com/api/v4/projects/18695663/releases/{tag_name}/assets/links"
    );

    get(url)
        .unwrap()
        .json::<Vec<AssetLink>>()
        .unwrap()
        .iter()
        .find(|link| link.direct_asset_url.contains("darwin"))
        .map(|link| AssetLink {
            direct_asset_url: link.direct_asset_url.to_string(),
            name: link.name.to_string(),
        })
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

    if let Some(asset_link) = get_asset_link(&value) {
        let direct_asset_url = &asset_link.direct_asset_url;
        println!("Downloading from {}...", direct_asset_url);
        let content = get(direct_asset_url).unwrap().bytes().unwrap();
        let filename = format!("~/Desktop/{}", asset_link.name);
        let mut output = File::create(tilde(&filename).to_string()).unwrap();
        io::copy(&mut content.as_ref(), &mut output).unwrap();
    } else {
        println!("No assets found.")
    }
}
