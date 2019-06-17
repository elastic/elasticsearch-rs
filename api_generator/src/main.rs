extern crate dialoguer;

use std::io::{Write, Read};
use std::fs::{File};
use std::path::Path;
use dialoguer::Input;

mod api_generator;
mod rest_spec;

fn main() {
    let download_dir = "rest_specs";
    let last_downloaded_version = "last_downloaded_version";
    let mut download_specs = false;
    let mut answer = String::new();
    let mut branch = String::new();
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
        let mut default_branch = String::new();

        if Path::new(last_downloaded_version).exists() {
            File::open(last_downloaded_version)
                .expect("Could not open last_downloaded_version file")
                .read_to_string(&mut default_branch)
                .expect("Could not read branch into string");
        } else {
            default_branch.push_str("master");
        }

        branch = Input::new()
            .default(default_branch.clone())
            .show_default(false)
            .with_prompt(format!("Branch to download specification from [default {}]", default_branch).as_str())
            .interact()
            .unwrap();

        rest_spec::download_specs(&branch, &download_dir);

        File::create(last_downloaded_version)
            .expect("failed to create last_downloaded_version file")
            .write(branch.as_bytes())
            .expect("unable to write branch to last_downloaded_version file");
    }

    let mut generate_code = true;
    answer = String::new();
    while answer != "y" && answer != "n" {
        answer = Input::new()
            .default(String::from("y"))
            .show_default(false)
            .with_prompt("Generate code from rest specifications [Y/n]")
            .interact()
            .unwrap()
            .to_lowercase();
        generate_code = answer == "y";
    }

    if generate_code {
        api_generator::generate(&branch, &download_dir);
    }
}