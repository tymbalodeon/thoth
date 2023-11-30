use owo_colors::OwoColorize;
use serde::Deserialize;

use super::{get_versions, list_versions, VersionStability, GITLAB_URL};

#[derive(Debug, Deserialize)]
pub struct DirectAssetUrl {
    pub direct_asset_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub links: Vec<DirectAssetUrl>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Link,
}

pub struct LilypondReleases {
    releases: <Vec<Release> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl LilypondReleases {
    pub fn get() -> reqwest::Result<Self> {
        Ok(Self {
            releases: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    fn try_next(&mut self) -> reqwest::Result<Option<Release>> {
        if let Some(release) = self.releases.next() {
            return Ok(Some(release));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "{GITLAB_URL}?page={}&per_page={}",
            self.page, self.per_page
        );
        let response = self.client.get(url).send()?;
        self.total = response
            .headers()
            .get("x-total")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u32>()
            .unwrap()
            .to_owned();
        self.releases = response.json::<Vec<Release>>()?.into_iter();

        Ok(self.releases.next())
    }
}

impl Iterator for LilypondReleases {
    type Item = reqwest::Result<Release>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(release)) => Some(Ok(release)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

pub fn list_remote(
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    let versions: Vec<String> = get_versions()
        .iter()
        .map(|release| release.bold().to_string())
        .collect();

    list_versions(versions, version_regex, stability);
}
