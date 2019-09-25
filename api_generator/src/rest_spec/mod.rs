extern crate pbr;
extern crate reqwest;

use pbr::ProgressBar;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::copy;
use std::path::PathBuf;

struct GitHubSpec {
    dir: String,
    branch: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Links {
    #[serde(rename = "self")]
    self_link: String,
    git: String,
    html: String,
}

#[derive(Deserialize, Debug)]
struct RestApiSpec {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "_links")]
    links: Links,
}

pub fn download_specs(branch: &str, download_dir: &PathBuf) {
    let spec_urls = [
        ("core".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/rest-api-spec/src/main/resources/rest-api-spec/api".to_string()),
        ("xpack".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/x-pack/plugin/src/test/resources/rest-api-spec/api".to_string())];

    let specs: Vec<GitHubSpec> = spec_urls
        .iter()
        .map(|(dir, template_url)| {
            let url = format!("{}?ref={}", template_url, branch);
            GitHubSpec {
                dir: dir.to_string(),
                branch: branch.to_string(),
                url,
            }
        })
        .collect();

    fs::create_dir_all(download_dir).unwrap();
    for spec in specs {
        download_endpoints(&spec, &download_dir);
    }
}

fn download_endpoints(spec: &GitHubSpec, download_dir: &PathBuf) {
    let mut response = reqwest::get(&spec.url).unwrap();
    let rest_api_specs: Vec<RestApiSpec> = response.json().unwrap();
    let max_name =
        rest_api_specs
            .iter()
            .fold(rest_api_specs[0].name.len(), |acc, rest_api_spec| {
                if rest_api_spec.name.len() > acc {
                    rest_api_spec.name.len()
                } else {
                    acc
                }
            })
            + 1;

    println!(
        "Downloading {} specs from {}",
        spec.dir,
        spec.branch
    );
    let mut pb = ProgressBar::new(rest_api_specs.len() as u64);

    // TODO: parallelize downloads
    for rest_api_spec in rest_api_specs {
        let mut download_path = download_dir.clone();
        download_path.push(rest_api_spec.name.as_str());

        pb.message(right_pad(rest_api_spec.name.as_str(), max_name).as_str());

        let mut json = reqwest::get(rest_api_spec.download_url.as_str())
            .expect("failed to download endpoint json");
        let mut file = File::create(download_path.to_string_lossy().into_owned())
            .expect("failed to create file");
        copy(&mut json, &mut file).expect("failed to copy response to file");

        pb.inc();
    }

    pb.finish_print(format!("Done downloading {} specs from {}", spec.dir, spec.branch).as_str());
}

fn right_pad(s: &str, pad: usize) -> String {
    let mut out = String::from(s);
    let len = s.len();
    if pad > len {
        for _ in 0..pad - len {
            out.push(' ');
        }
    }
    out
}
