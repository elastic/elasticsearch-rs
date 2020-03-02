use elasticsearch::{
    auth::Credentials, http::response::Response, http::transport::TransportBuilder,
    indices::IndicesExistsParts, params::Refresh, BulkOperation, BulkParts, Elasticsearch, Error,
    DEFAULT_ADDRESS,
};

use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::SingleNodeConnectionPool;
use reqwest::StatusCode;
use serde_json::json;
use sysinfo::SystemExt;
use url::Url;

/// Gets the address to the Elasticsearch instance from environment variables
/// and assumes an instance running locally on the default port otherwise
fn cluster_addr() -> String {
    match std::env::var("ES_TEST_SERVER") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Checks if Fiddler proxy process is running
fn running_proxy() -> bool {
    let system = sysinfo::System::new();
    !system.get_process_by_name("Fiddler").is_empty()
}

pub fn create_default_builder() -> TransportBuilder {
    create_builder(cluster_addr().as_str())
}

pub fn create_builder(addr: &str) -> TransportBuilder {
    let url = Url::parse(addr).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url.clone());
    let mut builder = TransportBuilder::new(conn_pool);
    // assume if we're running with HTTPS then authentication is also enabled and disable
    // certificate validation - we'll change this for tests that need to.
    if url.scheme() == "https" {
        builder = builder
            .auth(Credentials::Basic("elastic".into(), "changeme".into()))
            .cert_validation(CertificateValidation::None)
    }

    builder
}

pub fn create_default() -> Elasticsearch {
    create_for_url(cluster_addr().as_str())
}

pub fn create_for_url(url: &str) -> Elasticsearch {
    let builder = create_builder(url);
    create(builder)
}

pub fn create(mut builder: TransportBuilder) -> Elasticsearch {
    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build().unwrap();
    Elasticsearch::new(transport)
}

/// index some documents into a posts index. If the posts index already exists, do nothing.
///
/// As an async fn, this can end up running multiple times concurrently, and indexing documents
/// several times. In this instance, this is fine.
///
/// TODO: This is a temporary measure until https://github.com/elastic/elasticsearch-rs/issues/19 is implemented.
pub async fn index_documents(client: &Elasticsearch) -> Result<Response, Error> {
    let index = "posts";
    let exists_response = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

    if exists_response.status_code() == StatusCode::NOT_FOUND {
        let mut body: Vec<BulkOperation<_>> = vec![];
        for i in 1..11 {
            let op = BulkOperation::index(i.to_string(), json!({"title":"Elasticsearch"})).into();
            body.push(op);
        }

        client
            .bulk(BulkParts::Index(index))
            .body(body)
            .refresh(Refresh::WaitFor)
            .send()
            .await
    } else {
        Ok(exists_response)
    }
}
