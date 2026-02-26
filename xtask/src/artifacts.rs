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
use regex::Regex;
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

pub fn download(stack_version: &str) -> anyhow::Result<()> {
    let api_stack_version = if stack_version == "main" {
        "master"
    } else {
        let re = Regex::new(r"^\d+\.\d+$").expect("valid stack version regex");
        anyhow::ensure!(
            re.is_match(stack_version),
            "STACK_VERSION must be two numeric parts separated by a dot (e.g. 9.0); got {}",
            stack_version
        );
        stack_version
    };

    let spec_dir = ROOT_DIR.join("checkout").join(stack_version);
    let snapshot_url = format!(
        "https://artifacts-snapshot.elastic.co/elasticsearch/latest/{}.json",
        api_stack_version
    );
    println!("Downloading build info from {}", &snapshot_url);
    let artifacts = reqwest::get(&snapshot_url)?.json::<SnapshotRoot>()?;
    let manifest_url = artifacts.manifest_url;
    let version = artifacts.version;
    println!("Downloading manifest from {}", &manifest_url);

    let manifest = reqwest::get(&manifest_url)?.json::<Manifest>()?;
    let project = manifest
        .projects
        .get("elasticsearch")
        .with_context(|| "Project 'elasticsearch' not found")?;

    let specs_url = project
        .packages
        .get(&format!("rest-resources-zip-{}.zip", version))
        .with_context(|| "Package 'rest-resources-zip' not found")?
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
    let project_file = fs::File::create(project_path)?;
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
pub struct SnapshotRoot {
    pub version: String,
    pub build_id: String,
    pub manifest_url: String,
    // pub_summary_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Manifest {
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
