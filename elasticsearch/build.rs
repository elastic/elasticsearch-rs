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
extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    let version = version_meta().unwrap();
    match version.channel {
        Channel::Stable => {
            println!("cargo:rustc-cfg=RUSTC_IS_STABLE");
        }
        Channel::Beta => {
            println!("cargo:rustc-cfg=RUSTC_IS_BETA");
        }
        Channel::Nightly => {
            println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY");
        }
        Channel::Dev => {
            println!("cargo:rustc-cfg=RUSTC_IS_DEV");
        }
    }

    let semver = version.semver;
    let mut rustcv: String = format!("{}.{}.{}", semver.major, semver.minor, semver.patch);
    if version.channel != Channel::Stable {
        rustcv.push('p');
    }
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustcv);
}
