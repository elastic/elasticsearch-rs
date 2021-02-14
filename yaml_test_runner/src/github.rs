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
use flate2::read::GzDecoder;
use globset::Glob;
use io::Write;
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Response,
};
use std::{fs, fs::File, io, path::PathBuf};
use tar::{Archive, Entry};

/// Downloads the yaml tests if not already downloaded
pub fn download_test_suites(branch: &str, download_dir: &PathBuf) -> Result<(), failure::Error> {
    let mut last_downloaded_version = download_dir.clone();
    last_downloaded_version.push("last_downloaded_version");
    if last_downloaded_version.exists() {
        let version = fs::read_to_string(&last_downloaded_version)
            .expect("Unable to read last_downloaded_version of yaml tests");
        if version == branch {
            info!("Already downloaded yaml tests from {}", branch);
            return Ok(());
        }
    }

    info!("Downloading yaml tests from {}", branch);
    let url = format!(
        "https://api.github.com/repos/elastic/elasticsearch/tarball/{}",
        branch
    );
    let mut headers = HeaderMap::new();
    headers.append(
        USER_AGENT,
        HeaderValue::from_str(&format!("elasticsearch-rs/{}", env!("CARGO_PKG_NAME")))?,
    );
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client.get(&url).send()?;
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);

    let oss_test = Glob::new("**/rest-api-spec/src/main/resources/rest-api-spec/test/**/*.yml")?
        .compile_matcher();
    let xpack_test = Glob::new("**/x-pack/plugin/src/test/resources/rest-api-spec/test/**/*.yml")?
        .compile_matcher();

    for entry in archive.entries()? {
        let file = entry?;
        let path = file.path()?;
        if oss_test.is_match(&path) {
            write_test_file(download_dir, "free", file)?;
        } else if xpack_test.is_match(&path) {
            write_test_file(download_dir, "xpack", file)?;
        }
    }

    info!("Downloaded yaml tests from {}", &branch);
    File::create(last_downloaded_version)
        .expect("failed to create last_downloaded_version file")
        .write_all(branch.as_bytes())
        .expect("unable to write branch to last_downloaded_version file");

    Ok(())
}

fn write_test_file(
    download_dir: &PathBuf,
    suite_dir: &str,
    mut entry: Entry<GzDecoder<Response>>,
) -> Result<(), failure::Error> {
    let path = entry.path()?;

    let mut dir = {
        let mut dir = download_dir.clone();
        dir.push(suite_dir);
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
