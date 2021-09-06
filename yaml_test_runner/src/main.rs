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
#![recursion_limit = "256"]

extern crate api_generator;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quote;

extern crate simple_logger;

use anyhow::bail;
use clap::{App, Arg};
use log::LevelFilter;
use serde_json::Value;
use std::{fs, path::PathBuf, process::exit};

mod generator;
mod regex;
mod step;

use generator::TestSuite;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("ELASTICSEARCH_URL")
            .help("The url of a running Elasticsearch cluster. Used to determine the version, test suite and branch to use to compile tests")
            .required(true)
            .takes_value(true))
        .get_matches();

    let url = matches.value_of("url").expect("missing 'url' argument");
    let (branch, suite, version, sem_version) =
        match branch_suite_and_version_from_elasticsearch(url) {
            Ok(v) => v,
            Err(e) => {
                error!(
                    "Problem getting values from Elasticsearch at {}. {:?}",
                    url, e
                );
                exit(1);
            }
        };

    info!("Using version {}", &version);
    info!("Using branch {}", &branch);
    info!("Using test_suite {:?}", &suite);

    let stack_version = std::env::var("STACK_VERSION").expect("Missing STACK_VERSION env var");

    let valid_version = if let Some(pos) = stack_version.find(".x-SNAPSHOT") {
        &version[0..pos] == &stack_version[0..pos] && version.ends_with("-SNAPSHOT")
    } else {
        version != stack_version
    };

    if !valid_version {
        bail!(
            "ES server version {} is inconsistent with STACK_VERSION={}",
            version,
            stack_version
        );
    }

    let rest_specs_dir = PathBuf::from(format!("./checkout/{}/rest-api-spec/api", stack_version));
    if !rest_specs_dir.is_dir() {
        bail!("No specs found at {:?}", rest_specs_dir);
    }

    let download_dir = PathBuf::from(format!("./checkout/{}/rest-api-spec/test", stack_version));
    let generated_dir = PathBuf::from(format!("./{}/tests", env!("CARGO_PKG_NAME")));

    let api = api_generator::generator::read_api(&rest_specs_dir)?;

    // delete everything under the generated_dir except common dir
    if generated_dir.exists() {
        let entries = fs::read_dir(&generated_dir)?;
        for entry in entries {
            if let Ok(e) = entry {
                if let Ok(f) = e.file_type() {
                    if e.file_name() != "common" {
                        if f.is_dir() {
                            fs::remove_dir_all(e.path())?;
                        } else if f.is_file() {
                            fs::remove_file(e.path())?;
                        }
                    }
                }
            }
        }
    }

    generator::generate_tests_from_yaml(
        &api,
        &suite,
        &sem_version,
        &download_dir,
        &download_dir,
        &generated_dir,
    )?;

    Ok(())
}

fn branch_suite_and_version_from_elasticsearch(
    url: &str,
) -> Result<(String, TestSuite, String, semver::Version), failure::Error> {
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?;

    let suite = match std::env::var("TEST_SUITE") {
        Err(_) => panic!("Env var TEST_SUITE is not defined"),
        Ok(ref s) if s == "free" => TestSuite::Free,
        _ => TestSuite::XPack,
    };
    let mut response = client.get(url).send()?;
    let json: Value = response.json()?;
    let branch = json["version"]["build_hash"].as_str().unwrap().to_string();

    // any prerelease part needs to be trimmed because the semver crate only allows
    // a version with a prerelease to match against predicates, if at least one predicate
    // has a prerelease. See
    // https://github.com/steveklabnik/semver/blob/afa5fc853cb4d6d2b1329579e5528f86f3b550f9/src/version_req.rs#L319-L331
    let version = json["version"]["number"].as_str().unwrap().to_string();

    let sem_version =
        semver::Version::parse(version.trim_end_matches(|c: char| c.is_alphabetic() || c == '-'))?;

    Ok((branch, suite, version, sem_version))
}
