extern crate dialoguer;

extern crate api_generator;

use dialoguer::Input;
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Write,
};

use api_generator::generator;
use api_generator::rest_spec;

fn main() -> Result<(), failure::Error> {
    // This must be run from the src root directory, with cargo run -p api_generator
    let download_dir = fs::canonicalize(PathBuf::from("./api_generator/rest_specs"))?;
    let generated_dir = fs::canonicalize(PathBuf::from("./elasticsearch/src/generated"))?;
    let last_downloaded_version = PathBuf::from("./api_generator/rest_specs/last_downloaded_version");

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
            // delete existing generated files if the exist
            if generated_dir.exists() {
                fs::remove_dir_all(&generated_dir)?;
            }

            fs::create_dir_all(&generated_dir)?;
            generator::generate(&branch, &download_dir, &generated_dir)?;
        }
    }

    Ok(())
}
