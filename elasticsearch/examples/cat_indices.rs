use elasticsearch::auth::Credentials;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::{Elasticsearch, Error, DEFAULT_ADDRESS};
use sysinfo::SystemExt;
use url::Url;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let response = client
        .cat()
        .indices(CatIndicesParts::None)
        .v(true)
        .send()
        .await?;

    let text = response.text().await?;
    println!("{}", text);
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
