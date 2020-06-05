#[macro_use]
extern crate serde_json;

use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, Error, SearchParts, DEFAULT_ADDRESS,
};
use serde_json::Value;
use std::env;
use sysinfo::SystemExt;
use url::Url;

mod stack_overflow;
use stack_overflow::*;

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
    let response = client
        .search(SearchParts::Index(&["posts"]))
        .body(query)
        .pretty(true)
        .send()
        .await?;

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
                println!("{}", q.title);
                println!("https://stackoverflow.com/q/{}", q.id);
                println!();
                println!("{}", q.body);
                println!("{}", "-".repeat(50));
            }
            Post::Answer(a) => {
                println!("https://stackoverflow.com/a/{}", a.id);
                println!();
                println!("{}", a.body);
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
            "elastic".into()
        };

        let password = match url.password() {
            Some(p) => {
                let pass = p.to_string();
                url.set_password(None).unwrap();
                pass
            }
            None => "changeme".into(),
        };

        Some(Credentials::Basic(username, password))
    } else {
        None
    };

    let conn_pool = SingleNodeConnectionPool::new(url);
    let mut builder = TransportBuilder::new(conn_pool);

    builder = match credentials {
        Some(c) => builder.auth(c).cert_validation(CertificateValidation::None),
        None => builder,
    };

    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}
