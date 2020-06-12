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
extern crate api_generator;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quote;

extern crate simple_logger;

use clap::{App, Arg};
use log::Level;
use std::{fs, path::PathBuf, process::exit};

mod generator;
mod github;
mod regex;
mod step;

use generator::TestSuite;

fn main() -> Result<(), failure::Error> {
    simple_logger::init_with_level(Level::Info).unwrap();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("branch")
            .short("b")
            .long("branch")
            .value_name("BRANCH")
            .help("The git branch in the Elasticsearch repository from which to download yaml tests")
            .required(true)
            .default_value("master")
            .takes_value(true))
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("TOKEN")
            .help("The GitHub access token. Increases the rate limit to be able to download all yaml tests. Must be specified if not passed through environment variable")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("The path to the rest API specs. Required to build a representation of the client API.")
            .required(true)
            .takes_value(true))
        .get_matches();

    // Get the version from ELASTICSEARCH_VERSION environment variable, if set.
    // any prerelease part needs to be trimmed because the semver crate only allows
    // a version with a prerelease to match against predicates, if at least one predicate
    // has a prerelease. See
    // https://github.com/steveklabnik/semver/blob/afa5fc853cb4d6d2b1329579e5528f86f3b550f9/src/version_req.rs#L319-L331
    let (suite, version) = match std::env::var("ELASTICSEARCH_VERSION") {
        Ok(v) => {
            let suite = if v.contains("oss") {
                TestSuite::Oss
            } else {
                TestSuite::XPack
            };

            let v = v
                .split(':')
                .next_back()
                .unwrap()
                .trim_end_matches(|c: char| c.is_alphabetic() || c == '-');

            (suite, semver::Version::parse(v)?)
        }
        Err(_) => {
            error!("ELASTICSEARCH_VERSION environment variable must be set to compile tests");
            exit(1);
        }
    };

    info!("Using version {:?} to compile tests", &version);

    let branch = matches
        .value_of("branch")
        .expect("missing 'branch' argument");

    let token = match std::env::var("TOKEN") {
        Ok(v) => v,
        Err(_) => match matches.value_of("token") {
            Some(v) => v.into(),
            None => {
                error!("missing GitHub token. Either pass as TOKEN environment variable, or 'token' argument");
                exit(1);
            }
        },
    };
    let path = matches.value_of("path").expect("missing 'path' argument");
    let rest_specs_dir = PathBuf::from(path);
    let download_dir = PathBuf::from(format!("./{}/yaml", env!("CARGO_PKG_NAME")));
    let generated_dir = PathBuf::from(format!("./{}/tests", env!("CARGO_PKG_NAME")));

    github::download_test_suites(&token, branch, &download_dir)?;

    let mut last_downloaded_rest_spec_branch = rest_specs_dir.clone();
    last_downloaded_rest_spec_branch.push("last_downloaded_version");

    let mut download_rest_specs = true;
    if last_downloaded_rest_spec_branch.exists() {
        let version = fs::read_to_string(last_downloaded_rest_spec_branch)
            .expect("Could not read rest specs last_downloaded version into string");

        if version == branch {
            info!(
                "rest specs for branch {} already downloaded in {:?}",
                branch, &rest_specs_dir
            );
            download_rest_specs = false;
        }
    }

    if download_rest_specs {
        api_generator::rest_spec::download_specs(branch, &rest_specs_dir)?;
    }

    let api = api_generator::generator::read_api(branch, &rest_specs_dir)?;

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
        &version,
        &download_dir,
        &download_dir,
        &generated_dir,
    )?;

    Ok(())
}
