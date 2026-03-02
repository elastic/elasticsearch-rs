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

use clap::Parser;
use once_cell::sync::Lazy;
use std::env;
use std::ops::Deref;
use std::path;

pub mod artifacts;

/// elasticsearch-rs build helpers
#[derive(Parser, Debug)]
enum Cmd {
    /// Download Elasticsearch Rest specs and YAML tests
    /// based on the STACK_VERSION environment variable
    DownloadSpecs {},
}

fn main() -> anyhow::Result<()> {
    let opt = Cmd::parse();
    match opt {
        Cmd::DownloadSpecs {} => artifacts::download(STACK_VERSION.deref()),
    }
}

pub static ROOT_DIR: Lazy<path::PathBuf> = Lazy::new(|| {
    let mf = env::var("CARGO_MANIFEST_DIR").expect("Should be run using 'cargo xtask ...'");
    path::Path::new(&mf).parent().unwrap().to_owned()
});

pub static STACK_VERSION: Lazy<String> =
    Lazy::new(|| env::var("STACK_VERSION").expect("Missing STACK_VERSION environment variable"));
