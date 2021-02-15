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
extern crate dialoguer;

use api_generator::{generator, rest_spec};
use dialoguer::Input;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

fn main() -> Result<(), failure::Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    // This must be run from the repo root directory, with cargo make generate-api
    let download_dir = fs::canonicalize(PathBuf::from("./api_generator/rest_specs"))?;
    let generated_dir = fs::canonicalize(PathBuf::from("./elasticsearch/src"))?;
    let last_downloaded_version =
        PathBuf::from("./api_generator/rest_specs/last_downloaded_version");

    let mut download_specs = false;
    let mut answer = String::new();
    let default_branch = if last_downloaded_version.exists() {
        fs::read_to_string(&last_downloaded_version)?
    } else {
        String::from("master")
    };
    let mut branch = default_branch.clone();

    while answer != "y" && answer != "n" {
        answer = Input::new()
            .default(String::from("n"))
            .show_default(false)
            .with_prompt("Download rest specifications [y/N]")
            .interact()
            .unwrap()
            .to_lowercase();
        download_specs = answer == "y";
    }

    if download_specs {
        branch = Input::new()
            .default(default_branch.clone())
            .show_default(false)
            .with_prompt(
                format!(
                    "Branch to download specification from [default {}]",
                    default_branch
                )
                .as_str(),
            )
            .interact()
            .unwrap();

        fs::remove_dir_all(&download_dir)?;
        fs::create_dir_all(&download_dir)?;
        rest_spec::download_specs(&branch, &download_dir)?;
        File::create(&last_downloaded_version)?.write_all(branch.as_bytes())?;
    }

    // only offer to generate if there are downloaded specs
    if download_dir
        .read_dir()
        .map(|mut r| r.next().is_some())
        .unwrap_or(false)
    {
        let mut generate_code = true;
        answer = String::new();
        while answer != "y" && answer != "n" {
            answer = Input::new()
                .default(String::from("y"))
                .show_default(false)
                .with_prompt(
                    format!("Generate code from rest specifications {} [Y/n]", branch).as_str(),
                )
                .interact()
                .unwrap()
                .to_lowercase();
            generate_code = answer == "y";
        }

        if generate_code {
            // Delete previously generated files
            let mut generated = generated_dir.clone();
            generated.push(generator::GENERATED_TOML);

            if generated.exists() {
                let files =
                    toml::from_str::<generator::GeneratedFiles>(&fs::read_to_string(generated)?)?;

                for f in files.written {
                    let mut generated_file = generated_dir.clone();
                    generated_file.push(f);
                    let _ = fs::remove_file(generated_file); // ignore missing files
                }
            }

            // and generate!
            generator::generate(&branch, &download_dir, &generated_dir)?;
        }
    }

    Ok(())
}
