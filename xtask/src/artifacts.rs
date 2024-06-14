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
use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use flate2::read::GzDecoder;
use globset::{Glob, GlobSetBuilder};
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use tar::{Archive, Entry};

pub fn download(commit_hash: Option<String>, url: Option<String>) -> anyhow::Result<()> {
    // Get commit hash from ES if its URL has been provided
    let commit_hash = if let Some(url) = url {
        Some(get_es_commit_hash(url)?)
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
    if download_from_artifacts_api(&spec_dir, commit_hash.clone()).is_err() {
        println!("No build info artifacts API");
        download_from_github(&spec_dir, commit_hash)?;
    }

    Ok(())
}

fn download_from_github(spec_dir: &PathBuf, commit_hash: Option<String>) -> anyhow::Result<()> {
    let branch_url = format!(
        "https://api.github.com/repos/elastic/elasticsearch/tarball/{}",
        commit_hash
            .clone()
            .unwrap_or(format!("v{}", *STACK_VERSION))
    );
    println!("Downloading tarball from {}", &branch_url);
    let mut headers = HeaderMap::new();
    headers.append(USER_AGENT, HeaderValue::from_str("elasticsearch-rs")?);
    let client = reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client.get(&branch_url).send()?;
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);

    let oss_spec = Glob::new("**/rest-api-spec/src/main/resources/rest-api-spec/api/*.json")?
        .compile_matcher();
    let xpack_spec = Glob::new("**/x-pack/plugin/src/test/resources/rest-api-spec/api/*.json")?
        .compile_matcher();

    let oss_test = GlobSetBuilder::new()
        .add(Glob::new(
            "**/rest-api-spec/src/main/resources/rest-api-spec/test/**/*.yml",
        )?)
        .add(Glob::new(
            "**/rest-api-spec/src/yamlRestTest/resources/rest-api-spec/test/**/*.yml",
        )?)
        .build()?;
    let xpack_test = GlobSetBuilder::new()
        .add(Glob::new(
            "**/x-pack/plugin/src/test/resources/rest-api-spec/test/**/*.yml",
        )?)
        .add(Glob::new(
            "**/x-pack/plugin/src/yamlRestTest/resources/rest-api-spec/test/**/*.yml",
        )?)
        .build()?;

    if spec_dir.exists() {
        fs::remove_dir_all(&spec_dir).unwrap();
    }
    let rest_dir = spec_dir.join("rest-api-spec");
    let api_dir = rest_dir.join("api");
    let test_dir = rest_dir.join("test");
    fs::create_dir_all(&api_dir).unwrap();
    for entry in archive.entries()? {
        let file = entry?;
        let path = file.path()?;
        if oss_spec.is_match(&path) || xpack_spec.is_match(&path) {
            write_spec_file(&api_dir, file)?;
        } else if oss_test.is_match(&path) {
            write_test_file(&test_dir, "free", file)?;
        } else if xpack_test.is_match(&path) {
            write_test_file(&test_dir, "platinum", file)?;
        }
    }

    // Also write project metadata for reference
    let project_path = &spec_dir.join("elasticsearch.json");
    let project_file = fs::File::create(&project_path)?;
    serde_json::to_writer_pretty(
        project_file,
        &Project {
            branch: STACK_VERSION.deref().clone(),
            commit_hash: commit_hash.unwrap_or_default(),
            packages: HashMap::new(),
        },
    )?;
    Ok(())
}

fn download_from_artifacts_api(
    spec_dir: &PathBuf,
    commit_hash: Option<String>,
) -> anyhow::Result<()> {
    let artifacts_url = format!(
        "https://artifacts-api.elastic.co/v1/versions/{}",
        *STACK_VERSION
    );
    println!("Downloading build info from {}", &artifacts_url);
    let artifacts = reqwest::blocking::get(&artifacts_url)?.json::<Artifacts>()?;
    println!("Downloaded build info from {}", &artifacts_url);
    let project = find_project(&commit_hash, artifacts)?;
    let zip_resp = download_specs_zip(&project)?;
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

fn write_test_file(
    download_dir: &Path,
    suite: &str,
    mut entry: Entry<GzDecoder<Response>>,
) -> anyhow::Result<()> {
    let path = entry.path()?;
    let mut dir = {
        let mut dir = download_dir.to_path_buf();
        dir.push(suite);
        let parent = path.parent().unwrap().file_name().unwrap();
        dir.push(parent);
        dir
    };
    fs::create_dir_all(&dir)?;
    dir.push(path.file_name().unwrap());
    let mut file = File::create(&dir)?;
    io::copy(&mut entry, &mut file)?;
    Ok(())
}

fn write_spec_file(
    download_dir: &Path,
    mut entry: Entry<GzDecoder<Response>>,
) -> anyhow::Result<()> {
    let path = entry.path()?;
    let mut dir = download_dir.to_path_buf();
    dir.push(path.file_name().unwrap());
    let mut file = File::create(&dir)?;
    io::copy(&mut entry, &mut file)?;
    Ok(())
}

fn find_project(commit_hash: &Option<String>, artifacts: Artifacts) -> anyhow::Result<Project> {
    match &commit_hash {
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
            .cloned()
            .with_context(|| "Project 'elasticsearch' not found"),

        None => artifacts
            .version
            .builds
            .iter()
            .max_by_key(|build| build.start_time)
            .unwrap()
            .projects
            .get("elasticsearch")
            .cloned()
            .with_context(|| "Project 'elasticsearch' not found"),
    }
}

fn download_specs_zip(project: &Project) -> anyhow::Result<Bytes> {
    let specs_url = project
        .packages
        .get(&format!("rest-resources-zip-{}.zip", *STACK_VERSION))
        .with_context(|| "Package 'rest-resources-zip' not found")?
        .url
        .clone();

    println!("Downloading specs and yaml tests from {}", &specs_url);
    reqwest::blocking::get(&specs_url)?
        .bytes()
        .with_context(|| "could not get bytes")
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub branch: String,
    pub commit_hash: String,
    pub packages: HashMap<String, Package>,
    // commit_url
    // build_duration_seconds: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
