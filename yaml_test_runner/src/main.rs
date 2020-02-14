use clap::{App, Arg};
use std::path::PathBuf;

mod generator;
mod github;

fn main() {
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
        .get_matches();

    let branch = matches
        .value_of("branch")
        .expect("missing 'branch' argument");
    let token = matches.value_of("token").expect("missing 'token' argument");
    let download_dir = PathBuf::from("./yaml_test_runner/yaml");

    //github::download_test_suites(token, branch, &download_dir);

    generator::generate_tests_from_yaml(&download_dir).unwrap();
}
