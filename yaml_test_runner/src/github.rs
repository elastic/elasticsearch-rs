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
use io::Write;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::Formatter;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};

struct YamlTestSuite {
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
struct GitHubContent {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "_links")]
    links: Links,
}

/// Downloads the yaml tests if not already downloaded
pub fn download_test_suites(
    token: &str,
    branch: &str,
    download_dir: &PathBuf,
) -> Result<(), failure::Error> {
    let mut last_downloaded_version = download_dir.clone();
    last_downloaded_version.push("last_downloaded_version");
    if last_downloaded_version.exists() {
        let version = fs::read_to_string(&last_downloaded_version)
            .expect("Unable to read last_downloaded_version of yaml tests");
        if version == branch {
            info!("yaml tests for branch {} already downloaded", branch);
            return Ok(());
        }
    }

    let test_suite_map = [
        ("oss".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/rest-api-spec/src/main/resources/rest-api-spec/test".to_string()),
        ("xpack".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/x-pack/plugin/src/test/resources/rest-api-spec/test".to_string())];

    let test_suites: Vec<YamlTestSuite> = test_suite_map
        .iter()
        .map(|(dir, template_url)| {
            let url = format!("{}?ref={}", template_url, branch);
            YamlTestSuite {
                dir: dir.to_string(),
                branch: branch.to_string(),
                url,
            }
        })
        .collect();

    let mut headers = HeaderMap::new();
    let token_value = format!("token {}", token);
    headers.append(AUTHORIZATION, HeaderValue::from_str(&token_value)?);
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    // delete existing yaml tests
    if download_dir.exists() {
        fs::remove_dir_all(&download_dir)?;
    }

    fs::create_dir_all(download_dir)?;

    for suite in test_suites {
        download_tests(&client, &suite, &download_dir)?;
    }

    File::create(last_downloaded_version)
        .expect("failed to create last_downloaded_version file")
        .write_all(branch.as_bytes())
        .expect("unable to write branch to last_downloaded_version file");

    Ok(())
}

fn download_tests(
    client: &reqwest::Client,
    suite: &YamlTestSuite,
    download_dir: &PathBuf,
) -> Result<(), DownloadError> {
    let suite_dir = {
        let mut d = download_dir.clone();
        d.push(&suite.dir);
        d
    };

    fs::create_dir_all(&suite_dir)?;
    info!("Downloading {} tests from {}", &suite.dir, &suite.branch);
    download(client, &suite.url, &suite_dir)?;
    info!(
        "Done downloading {} tests from {}",
        &suite.dir, &suite.branch
    );

    Ok(())
}

fn download(
    client: &reqwest::Client,
    url: &str,
    download_dir: &PathBuf,
) -> Result<(), DownloadError> {
    let mut response = client.get(url).send()?;

    let remaining_rate_limit: i32 = response
        .headers()
        .get("X-RateLimit-Remaining")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    if remaining_rate_limit < 10 {
        warn!("Remaining rate limit: {}", remaining_rate_limit);
    }

    let contents: Vec<GitHubContent> = response.json()?;
    for content in contents {
        let content_path = {
            let mut d = download_dir.clone();
            d.push(&content.name);
            d
        };

        match content.ty.as_str() {
            "file" => {
                let mut file = File::create(content_path)?;
                // no need to send the token for downloading content
                let mut file_response = reqwest::get(&content.download_url.unwrap())?;
                io::copy(&mut file_response, &mut file)?;
            }
            "dir" => {
                fs::create_dir_all(&content_path)?;
                download(client, &content.url, &content_path)?;
            }
            t => {
                return Err(DownloadError::InvalidType(format!(
                    "Unexpected GitHub content type: {}",
                    t
                )))
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum DownloadError {
    IoErr(io::Error),
    HttpError(reqwest::Error),
    InvalidType(String),
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DownloadError::IoErr(err) => write!(f, "IoErr {}", err),
            DownloadError::HttpError(err) => write!(f, "HttpError {}", err),
            DownloadError::InvalidType(s) => write!(f, "InvalidType {}", s),
        }
    }
}

impl StdError for DownloadError {
    #[allow(warnings)]
    fn description(&self) -> &str {
        match self {
            DownloadError::IoErr(err) => err.description(),
            DownloadError::HttpError(err) => err.description(),
            DownloadError::InvalidType(s) => s.as_ref(),
        }
    }
}

impl From<io::Error> for DownloadError {
    fn from(e: io::Error) -> Self {
        DownloadError::IoErr(e)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(e: reqwest::Error) -> Self {
        DownloadError::HttpError(e)
    }
}
