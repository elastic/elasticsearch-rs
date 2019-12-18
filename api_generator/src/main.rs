extern crate dialoguer;
#[macro_use]
extern crate quote;

use dialoguer::Input;
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

mod api_generator;
mod error;
mod rest_spec;

fn main() {
    // This must be run from the src root directory, with cargo run -p api_generator
    let download_dir = fs::canonicalize(PathBuf::from("./api_generator/rest_specs")).unwrap();
    let generated_dir = fs::canonicalize(PathBuf::from("./elasticsearch/src/generated")).unwrap();
    let last_downloaded_version = "./api_generator/last_downloaded_version";

    let mut download_specs = false;
    let mut answer = String::new();
    let default_branch = if Path::new(last_downloaded_version).exists() {
        fs::read_to_string(last_downloaded_version).expect("Could not read branch into string")
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

        fs::remove_dir_all(&download_dir).unwrap();
        rest_spec::download_specs(&branch, &download_dir);

        File::create(last_downloaded_version)
            .expect("failed to create last_downloaded_version file")
            .write_all(branch.as_bytes())
            .expect("unable to write branch to last_downloaded_version file");
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
            // delete existing generated files if the exist
            if generated_dir.exists() {
                fs::remove_dir_all(&generated_dir).unwrap();
            }

            fs::create_dir_all(&generated_dir).unwrap();

            api_generator::generate(&branch, &download_dir, &generated_dir).unwrap();
        }
    }
}
