use crate::commands::{
    lilypond::{get_version_stability, get_versions},
    table::print_table,
};

use super::VersionStability;

use itertools::{EitherOrBoth::*, Itertools};
use owo_colors::OwoColorize;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
}

struct LilypondReleases {
    project_id: String,
    releases: <Vec<Release> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl LilypondReleases {
    fn get() -> reqwest::Result<Self> {
        Ok(LilypondReleases {
            project_id: "18695663".to_string(),
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
            "https://gitlab.com/api/v4/projects/{}/releases?page={}&per_page={}",
            self.project_id,
            self.page,
            self.per_page
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

pub fn get_releases() -> Vec<String> {
    let mut releases = vec![];

    for release in LilypondReleases::get().unwrap() {
        releases.push(release.unwrap().tag_name.to_string());
    }

    releases
        .iter()
        .map(|release| release.replace(['v', '"'], "").replace("release/", ""))
        .collect()
}

pub fn list_remote(
    version_regex: &Option<String>,
    stability: &Option<VersionStability>,
) {
    let mut releases: Vec<String> = get_releases()
        .iter()
        .map(|release| release.bold().to_string())
        .collect();

    if let Some(stability) = stability {
        releases = releases
            .iter()
            .filter(|version| {
                get_version_stability(version).unwrap() == *stability
            })
            .map(|version| version.to_string())
            .collect();
    }

    if let Some(regex) = version_regex {
        let re = Regex::new(regex).unwrap();

        releases = releases
            .iter()
            .filter(|version| re.is_match(version))
            .map(|version| version.to_string())
            .collect();
    }

    let stable = get_versions(&releases, VersionStability::Stable);
    let unstable = get_versions(&releases, VersionStability::Unstable);

    let mut titles = vec![];

    if !stable.is_empty() {
        titles.push("Stable".italic().green().to_string())
    }

    if !unstable.is_empty() {
        titles.push("Unstable".italic().yellow().to_string())
    }

    let mut rows: Vec<Vec<String>> = vec![];

    if !stable.is_empty() && !unstable.is_empty() {
        for pair in stable.iter().zip_longest(unstable.iter()) {
            match pair {
                Both(stable, unstable) => {
                    rows.push(vec![stable.to_string(), unstable.to_string()])
                }
                Left(stable) => {
                    rows.push(vec![stable.to_string(), "".to_string()])
                }
                Right(unstable) => {
                    rows.push(vec!["".to_string(), unstable.to_string()])
                }
            }
        }
    } else if !stable.is_empty() {
        for version in stable.iter() {
            rows.push(vec![version.to_string()]);
        }
    } else if !unstable.is_empty() {
        for version in unstable.iter() {
            rows.push(vec![version.to_string()]);
        }
    }

    print_table(titles, rows);
}
