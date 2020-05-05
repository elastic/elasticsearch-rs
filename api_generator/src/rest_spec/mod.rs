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
extern crate reqwest;

mod parallel_downloading;

use parallel_downloading::download_specs_to_dir;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

struct GitHubSpec {
    dir: String,
    branch: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Links {
    #[serde(rename = "self")]
    self_link: String,
    git: String,
    html: String,
}

#[derive(Deserialize, Debug)]
struct RestApiSpec {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "_links")]
    links: Links,
}

pub fn download_specs(branch: &str, download_dir: &PathBuf) {
    let spec_urls = [
        ("core".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/rest-api-spec/src/main/resources/rest-api-spec/api".to_string()),
        ("xpack".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/x-pack/plugin/src/test/resources/rest-api-spec/api".to_string())];

    let specs: Vec<GitHubSpec> = spec_urls
        .iter()
        .map(|(dir, template_url)| {
            let url = format!("{}?ref={}", template_url, branch);
            GitHubSpec {
                dir: dir.to_string(),
                branch: branch.to_string(),
                url,
            }
        })
        .collect();

    fs::create_dir_all(download_dir).unwrap();
    for spec in specs {
        download_endpoints(&spec, &download_dir);
    }
}

fn download_endpoints(spec: &GitHubSpec, download_dir: &PathBuf) {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(concat!("RustApiGenerator/", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    let response = client.get(&spec.url).send().unwrap();
    let rest_api_specs: Vec<RestApiSpec> = response.json().unwrap();
    println!("Downloading {} specs from {}", spec.dir, spec.branch);
    download_specs_to_dir(client, rest_api_specs.as_slice(), download_dir).unwrap();
    println!("Done downloading {} specs from {}", spec.dir, spec.branch);
}
