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
use self::reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use flate2::read::GzDecoder;
use globset::Glob;
use reqwest::Response;
use std::{fs::File, io, path::PathBuf};
use tar::{Archive, Entry};

pub fn download_specs(branch: &str, download_dir: &PathBuf) -> anyhow::Result<()> {
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

    let oss_spec = Glob::new("**/rest-api-spec/src/main/resources/rest-api-spec/api/*.json")?
        .compile_matcher();
    let xpack_spec = Glob::new("**/x-pack/plugin/src/test/resources/rest-api-spec/api/*.json")?
        .compile_matcher();

    for entry in archive.entries()? {
        let file = entry?;
        let path = file.path()?;
        if oss_spec.is_match(&path) || xpack_spec.is_match(&path) {
            write_spec_file(download_dir, file)?;
        }
    }

    Ok(())
}

fn write_spec_file(
    download_dir: &PathBuf,
    mut entry: Entry<GzDecoder<Response>>,
) -> anyhow::Result<()> {
    let path = entry.path()?;
    let mut dir = download_dir.clone();
    dir.push(path.file_name().unwrap());
    let mut file = File::create(&dir)?;
    io::copy(&mut entry, &mut file)?;

    Ok(())
}
