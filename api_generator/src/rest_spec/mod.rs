extern crate reqwest;

mod parallel_downloading;

use parallel_downloading::download_specs_to_dir;
use serde::Deserialize;
use std::fs;
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

    println!("Downloading {} specs from {}", spec.dir, spec.branch);
    download_specs_to_dir(rest_api_specs.as_slice(), download_dir).unwrap();
    println!("Done downloading {} specs from {}", spec.dir, spec.branch);
}
