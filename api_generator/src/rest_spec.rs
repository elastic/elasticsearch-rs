extern crate pbr;
extern crate reqwest;
extern crate select;

use std::io::{copy, stdout, Write};
use std::fs::{self, File};
use pbr::ProgressBar;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class};

struct Spec {
    dir: String,
    branch: String,
    url: String
}

impl Spec {
    fn download_url(&self, endpoint: &str) -> String {
        let mut download_url = self.url
            .replace("github.com", "raw.githubusercontent.com")
            .replace("tree/", "");

        download_url.push_str("/");
        download_url.push_str(endpoint);
        return download_url;
    }
}

pub fn download_specs(branch : &str, download_dir : &str) {
    let spec_urls = [
        (String::from("core"), String::from("https://github.com/elastic/elasticsearch/tree/{branch}/rest-api-spec/src/main/resources/rest-api-spec/api")),
        (String::from("xpack"), String::from("https://github.com/elastic/elasticsearch/tree/{branch}/x-pack/plugin/src/test/resources/rest-api-spec/api"))];

    let specs : Vec<Spec> = spec_urls.iter().map(|(dir, template_url)| {
        let url = template_url.replace("{branch}", branch);
        Spec {
            dir: dir.to_string(),
            branch: branch.to_string(),
            url
        }
    }).collect();

    for spec in specs {
        fs::create_dir_all(format!("{}/{}", download_dir, spec.dir)).unwrap();
        download_endpoints(&spec, &download_dir);
    }
}

fn download_endpoints(spec : &Spec, download_dir : &str) {
    let resp = reqwest::get(&spec.url).unwrap();
    let document = Document::from_read(resp).unwrap();
    let nodes: Vec<Node> = document.find(Class("js-navigation-open")).filter(|node| {
        let text = node.text();
        return !text.is_empty() && text.ends_with(".json");
    }).collect();

    let max_name = nodes.iter().fold(nodes[0].text().len(), |acc, &node| {
        let text = node.text();
        if text.len() > acc {
            text.len()
        } else {
            acc
        }
    }) + 1;

    writeln!(stdout(), "Downloading {} specs from {}", spec.dir, spec.branch).unwrap();
    let mut pb = ProgressBar::new(nodes.len() as u64);

    for node in nodes {
        let endpoint_name = node.text();
        let url = spec.download_url(endpoint_name.as_str());
        let path = format!("{}/{}/{}", download_dir, spec.dir, endpoint_name);
        pb.message(right_pad(endpoint_name.as_str(), max_name).as_str());
        let mut json = reqwest::get(url.as_str()).expect("failed to download endpoint json");
        let mut file = File::create(path).expect("failed to create file");
        copy(&mut json, &mut file).expect("failed to copy response to file");
        pb.inc();
    }
    pb.finish_print(format!("Done downloading {} specs from {}", spec.dir, spec.branch).as_str());
}

fn right_pad(s: &str, pad: usize) -> String {
    let mut out = String::from(s);
    let len = s.len();
    if pad > len {
        for _ in 0..pad-len {
            out.push(' ');
        }
    }
    out
}