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

use anyhow::bail;
use once_cell::sync::Lazy;
use std::env;
use std::ops::Deref;
use std::path;
use structopt::StructOpt;

pub mod artifacts;

/// elasticsearch-rs build helpers
#[derive(StructOpt, Debug)]
enum Cmd {
    /// Download Elasticsearch Rest specs and YAML tests
    DownloadSpecs {
        /// Specific commit hash to download
        #[structopt(long, short)]
        commit_hash: Option<String>,
        /// Get the commit hash to download from a running ES server
        #[structopt(long, short)]
        url: Option<String>,
    },
    /// Get the commit hash of Elasticsearch running at <url>
    EsCommitHash {
        /// URL of the Elasticsearch server
        #[structopt(long, short)]
        url: String,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Cmd::from_args();
    match opt {
        Cmd::DownloadSpecs { commit_hash, url } => artifacts::download(commit_hash, url),
        Cmd::EsCommitHash { url } => {
            print!("{}", get_es_commit_hash(url)?);
            Ok(())
        }
    }
}

pub static ROOT_DIR: Lazy<path::PathBuf> = Lazy::new(|| {
    let mf = env::var("CARGO_MANIFEST_DIR").expect("Should be run using 'cargo xtask ...'");
    path::Path::new(&mf).parent().unwrap().to_owned()
});

pub static STACK_VERSION: Lazy<String> =
    Lazy::new(|| env::var("STACK_VERSION").expect("Missing STACK_VERSION environment variable"));

pub fn get_es_commit_hash(url: String) -> anyhow::Result<String> {
    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(url).send()?;
    if response.status() != reqwest::StatusCode::OK {
        bail!(
            "Cannot fetch ES information - status code {}",
            response.status()
        )
    }

    let json: serde_json::Value = response.json()?;
    let branch = json["version"]["build_hash"].as_str().unwrap().to_string();

    Ok(branch)
}
