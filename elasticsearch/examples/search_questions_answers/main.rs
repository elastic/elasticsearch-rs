#[macro_use]
extern crate serde_json;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, Error, SearchParts, DEFAULT_ADDRESS,
};
use serde_json::Value;
use std::env;
use sysinfo::SystemExt;
use url::Url;
mod stack_overflow;
use elasticsearch::http::response::Response;
use stack_overflow::*;
use textwrap::fill;

static POSTS_INDEX: &'static str = "posts";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let query = if args.len() < 2 {
        json!({
            "query": {
                "match_all": {}
            }
        })
    } else {
        json!({
            "query": {
                "match": {
                    "body": {
                        "query": args[1],
                        "operator": "and"
                    }
                }
            }
        })
    };

    let client = create_client()?;
    let mut response = client
        .search(SearchParts::Index(&[POSTS_INDEX]))
        .body(query)
        .pretty(true)
        .send()
        .await?;

    // turn the response into an Error if status code is unsuccessful
    response = response.error_for_status_code()?;

    let json: Value = response.json().await?;
    let posts: Vec<Post> = json["hits"]["hits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| serde_json::from_value(h["_source"].clone()).unwrap())
        .collect();

    for post in posts {
        match post {
            Post::Question(q) => {
                println!("{} - https://stackoverflow.com/q/{}", q.title, q.id);
                println!();
                println!("{}", fill(&q.body, 80));
                println!("{}", "-".repeat(50));
            }
            Post::Answer(a) => {
                println!("https://stackoverflow.com/a/{}", a.id);
                println!();
                println!("{}", fill(&a.body, 80));
                println!("{}", "-".repeat(30));
            }
        }
    }

    Ok(())
}

fn create_client() -> Result<Elasticsearch, Error> {
    fn cluster_addr() -> String {
        match std::env::var("ES_TEST_SERVER") {
            Ok(server) => server,
            Err(_) => DEFAULT_ADDRESS.into(),
        }
    }

    /// Determines if Fiddler.exe proxy process is running
    fn running_proxy() -> bool {
        let system = sysinfo::System::new();
        !system.get_process_by_name("Fiddler").is_empty()
    }

    let mut url = Url::parse(cluster_addr().as_ref()).unwrap();

    // if the url is https and specifies a username and password, remove from the url and set credentials
    let credentials = if url.scheme() == "https" {
        let username = if !url.username().is_empty() {
            let u = url.username().to_string();
            url.set_username("").unwrap();
            u
        } else {
            std::env::var("ES_USERNAME").unwrap_or_else(|_| "elastic".into())
        };

        let password = match url.password() {
            Some(p) => {
                let pass = p.to_string();
                url.set_password(None).unwrap();
                pass
            }
            None => std::env::var("ES_PASSWORD").unwrap_or_else(|_| "changeme".into()),
        };

        Some(Credentials::Basic(username, password))
    } else {
        None
    };

    let conn_pool = SingleNodeConnectionPool::new(url);
    let mut builder = TransportBuilder::new(conn_pool);

    builder = match credentials {
        Some(c) => {
            builder = builder.auth(c);

            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            {
                builder = builder.cert_validation(CertificateValidation::None);
            }

            builder
        }
        None => builder,
    };

    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}
