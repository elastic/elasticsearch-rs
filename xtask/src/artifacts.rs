/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use super::*;
use anyhow::Context;
use chrono::{DateTime, FixedOffset};
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn download(commit_hash: Option<String>, url: Option<String>) -> anyhow::Result<()> {
    // Get commit hash from ES if its URL has been provided
    let commit_hash = if let Some(url) = url {
        Some(super::get_es_commit_hash(url)?)
    } else {
        commit_hash
    };

    // Check if it's already there
    if let Some(expected_hash) = &commit_hash {
        if let Some(project) = read_local_project()? {
            if &project.commit_hash == expected_hash {
                println!("Specs were already downloaded.");
                return Ok(());
            }
        }
    }

    let spec_dir = ROOT_DIR.join("checkout").join(STACK_VERSION.deref());

    let artifacts_url = format!(
        "https://artifacts-api.elastic.co/v1/versions/{}",
        *STACK_VERSION
    );
    println!("Downloading build info from {}", &artifacts_url);

    let artifacts = reqwest::get(&artifacts_url)?.json::<Artifacts>()?;

    let project = match &commit_hash {
        Some(hash) => artifacts
            .version
            .builds
            .iter()
            .find(|build| {
                build
                    .projects
                    .get("elasticsearch")
                    .filter(|es| &es.commit_hash == hash)
                    .is_some()
            })
            .with_context(|| format!("Cannot find commit hash {}", hash))?
            .projects
            .get("elasticsearch")
            .unwrap(), // ES is guaranteed to be there

        None => artifacts
            .version
            .builds
            .iter()
            .max_by_key(|build| build.start_time)
            .unwrap()
            .projects
            .get("elasticsearch")
            .with_context(|| "Project 'elasticsearch' not found")?,
    };

    let specs_url = project
        .packages
        .iter()
        .find(|(name, _)| name.starts_with("rest-resources-zip-"))
        .with_context(|| "Package 'rest-resources-zip' not found")?
        .1
        .url
        .clone();

    println!("Downloading specs and yaml tests from {}", &specs_url);
    let zip_resp = reqwest::get(&specs_url)?.bytes()?;

    if spec_dir.exists() {
        fs::remove_dir_all(&spec_dir).unwrap();
    }
    fs::create_dir_all(&spec_dir).unwrap();
    zip::ZipArchive::new(io::Cursor::new(zip_resp))?.extract(&spec_dir)?;

    // Also write project metadata for reference
    let project_path = &spec_dir.join("elasticsearch.json");
    let project_file = fs::File::create(&project_path)?;
    serde_json::to_writer_pretty(project_file, &project)?;

    Ok(())
}

pub fn read_local_project() -> anyhow::Result<Option<Project>> {
    let spec_dir = ROOT_DIR.join("checkout").join(STACK_VERSION.deref());
    let project_path = &spec_dir.join("elasticsearch.json");

    if project_path.exists() {
        Ok(serde_json::from_reader(fs::File::open(project_path)?)?)
    } else {
        Ok(None)
    }
}

#[derive(Deserialize, Debug)]
pub struct Artifacts {
    pub version: Version,
    // manifests
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
    pub builds: Vec<Build>,
}

#[derive(Deserialize, Debug)]
pub struct Build {
    pub projects: HashMap<String, Project>,
    #[serde(with = "rfc2822_format")]
    pub start_time: DateTime<FixedOffset>,
    pub release_branch: String,
    pub version: String,
    pub branch: String,
    pub build_id: String,
    // end_time: String,
    // prefix: String,
    // manifest_version: String,
    // build_duration_seconds: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub branch: String,
    pub commit_hash: String,
    pub packages: HashMap<String, Package>,
    // commit_url
    // build_duration_seconds: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub url: String,
    pub sha_url: String,
    pub asc_url: String,
    pub architecture: Option<String>,

    #[serde(rename = "type")]
    pub type_: String,
    pub classifier: Option<String>,
    pub attributes: Option<Attributes>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub internal: Option<String>,
    pub artifact_id: Option<String>,
    pub oss: Option<String>,
    pub group: Option<String>,
}

#[allow(dead_code)]
mod rfc2822_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc2822();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc2822(&s).map_err(serde::de::Error::custom)
    }
}
