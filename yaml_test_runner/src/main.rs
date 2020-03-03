#[macro_use]
extern crate quote;

extern crate api_generator;

use clap::{App, Arg};
use std::path::PathBuf;
use std::fs;

mod generator;
mod github;

#[cfg(test)]
mod generated;

pub mod client;

fn main() -> Result<(), failure::Error> {
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
            .help("The GitHub access token. Required to increase the rate limit to be able to download all yaml tests")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("The path to the rest API specs. Required to build a representation of the client API.")
            .required(true)
            .takes_value(true))
        .get_matches();

    let branch = matches
        .value_of("branch")
        .expect("missing 'branch' argument");
    let token = matches.value_of("token").expect("missing 'token' argument");
    let path = matches.value_of("path").expect("missing 'path' argument");
    let rest_specs_dir = PathBuf::from(path);
    let download_dir = PathBuf::from("./yaml_test_runner/yaml");
    let generated_dir = PathBuf::from("./yaml_test_runner/src/generated");

    github::download_test_suites(token, branch, &download_dir);

    let mut last_downloaded_version = rest_specs_dir.clone();
    last_downloaded_version.push("last_downloaded_version");

    let mut download_rest_specs = true;
    if last_downloaded_version.exists() {
        let version = fs::read_to_string(last_downloaded_version)
            .expect("Could not rest specs last_downloaded version into string");

        if version == branch {
            println!("rest specs for branch {} already downloaded in {:?}", branch, &rest_specs_dir);
            download_rest_specs = false;
        }
    }

    if download_rest_specs {
        api_generator::rest_spec::download_specs(branch, &rest_specs_dir)?;
    }

    let api = api_generator::generator::read_api(branch, &rest_specs_dir)?;

    // delete existing generated files first
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir)?;
    }
    generator::generate_tests_from_yaml(&api, &download_dir, &download_dir, &generated_dir)?;

    Ok(())
}
