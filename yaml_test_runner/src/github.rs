use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::Formatter;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};

struct YamlTestSuite {
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
struct GitHubContent {
    name: String,
    path: String,
    sha: String,
    size: i32,
    url: String,
    html_url: String,
    git_url: String,
    download_url: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "_links")]
    links: Links,
}

pub fn download_test_suites(token: &str, branch: &str, download_dir: &PathBuf) {
    let test_suite_map = [
        ("oss".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/rest-api-spec/src/main/resources/rest-api-spec/test".to_string()),
        ("xpack".to_string(), "https://api.github.com/repos/elastic/elasticsearch/contents/x-pack/plugin/src/test/resources/rest-api-spec/test".to_string())];

    let test_suites: Vec<YamlTestSuite> = test_suite_map
        .iter()
        .map(|(dir, template_url)| {
            let url = format!("{}?ref={}", template_url, branch);
            YamlTestSuite {
                dir: dir.to_string(),
                branch: branch.to_string(),
                url,
            }
        })
        .collect();

    let mut headers = HeaderMap::new();
    let token_value = format!("token {}", token);
    headers.append(AUTHORIZATION, HeaderValue::from_str(&token_value).unwrap());
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    fs::create_dir_all(download_dir).unwrap();
    for suite in test_suites {
        download_tests(&client, &suite, &download_dir).unwrap();
    }
}

fn download_tests(
    client: &reqwest::Client,
    suite: &YamlTestSuite,
    download_dir: &PathBuf,
) -> Result<(), DownloadError> {
    let suite_dir = {
        let mut d = download_dir.clone();
        d.push(&suite.dir);
        d
    };

    fs::create_dir_all(&suite_dir)?;
    println!("Downloading {} tests from {}", &suite.dir, &suite.branch);
    download(client, &suite.url, &suite_dir)?;
    println!(
        "Done downloading {} tests from {}",
        &suite.dir, &suite.branch
    );

    Ok(())
}

fn download(
    client: &reqwest::Client,
    url: &str,
    download_dir: &PathBuf,
) -> Result<(), DownloadError> {
    let mut response = client.get(url).send()?;

    let remaining_rate_limit: i32 = response
        .headers()
        .get("X-RateLimit-Remaining")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();

    if remaining_rate_limit < 10 {
        println!("Remaining rate limit: {}", remaining_rate_limit);
    }

    let contents: Vec<GitHubContent> = response.json()?;
    for content in contents {
        let content_path = {
            let mut d = download_dir.clone();
            d.push(&content.name);
            d
        };

        match content.ty.as_str() {
            "file" => {
                let mut file = File::create(content_path)?;
                // no need to send the token for downloading content
                let mut file_response = reqwest::get(&content.download_url.unwrap())?;
                io::copy(&mut file_response, &mut file)?;
            }
            "dir" => {
                fs::create_dir_all(&content_path)?;
                download(client, &content.url, &content_path)?;
            }
            t => panic!(format!("Unexpected GitHub content type: {}", t)),
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum DownloadError {
    IoErr(io::Error),
    HttpError(reqwest::Error),
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DownloadError::IoErr(err) => write!(f, "IoErr {}", err),
            DownloadError::HttpError(err) => write!(f, "HttpError {}", err),
        }
    }
}

impl StdError for DownloadError {
    #[allow(warnings)]
    fn description(&self) -> &str {
        match self {
            DownloadError::IoErr(err) => err.description(),
            DownloadError::HttpError(err) => err.description(),
        }
    }
}

impl From<io::Error> for DownloadError {
    fn from(e: io::Error) -> Self {
        DownloadError::IoErr(e)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(e: reqwest::Error) -> Self {
        DownloadError::HttpError(e)
    }
}
