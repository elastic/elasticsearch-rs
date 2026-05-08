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
use anyhow::anyhow;
use git2::build::RepoBuilder;
use std::fs;

pub fn clone(stack_version: &str) -> anyhow::Result<()> {
    let major_minor_version = stack_version.split(".").collect::<Vec<&str>>()[0..2].join(".");
    let yaml_tests_url = String::from("https://github.com/elastic/elasticsearch-clients-tests");
    let yaml_tests_dir = ROOT_DIR.join("checkout/elasticsearch-clients-tests");
    if yaml_tests_dir.exists() {
        fs::remove_dir_all(&yaml_tests_dir).unwrap();
    }
    let mut builder = RepoBuilder::new();
    builder.branch(major_minor_version.as_str());
    if let Err(e) = builder.clone(yaml_tests_url.as_str(), yaml_tests_dir.as_path()) {
        Err(anyhow!("Failed to clone YAML tests repository: {}.", e))
    } else {
        Ok(())
    }
}
