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

use anyhow::{bail, Context};
use api_generator::generator;
use std::{fs, path::PathBuf};

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let stack_version = std::env::var("STACK_VERSION").context("Missing STACK_VERSION env var")?;

    // This must be run from the repo root directory, with cargo make generate-api
    let download_dir = PathBuf::from(&format!("./checkout/{}/rest-api-spec/api", stack_version));

    if !download_dir.is_dir() {
        bail!("No specs found at {:?}", download_dir);
    }

    // let download_dir = fs::canonicalize(PathBuf::from("./api_generator/rest_specs"))?;
    let generated_dir = fs::canonicalize(PathBuf::from("./elasticsearch/src"))?;

    // Delete previously generated files
    let mut generated = generated_dir.clone();
    generated.push(generator::GENERATED_TOML);
    if generated.exists() {
        let files = toml::from_str::<generator::GeneratedFiles>(&fs::read_to_string(generated)?)?;

        for f in files.written {
            let mut generated_file = generated_dir.clone();
            generated_file.push(f);
            let _ = fs::remove_file(generated_file); // ignore missing files
        }
    }

    // and generate!
    generator::generate(&download_dir, &generated_dir)?;

    Ok(())
}
