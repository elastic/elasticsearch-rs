use elasticsearch::{
    indices::IndicesExistsUrlParts, BulkUrlParts, ConnectionBuilder, Credentials, Elasticsearch,
    ElasticsearchError, ElasticsearchResponse, JsonBody, Refresh, DEFAULT_ADDRESS,
};

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

pub fn create_conn_builder(addr: &str) -> ConnectionBuilder {
    let url = Url::parse(addr).unwrap();
    let mut builder = ConnectionBuilder::new(url.clone());
    // assume if we're running with HTTPS then authentication is also enabled
    if url.scheme() == "https" {
        builder = builder.auth(Credentials::Basic("elastic".into(), "changeme".into()))
    }

    builder
}

pub fn create_default() -> Elasticsearch {
    create_for_url(cluster_addr().as_str())
}

pub fn create_for_url(url: &str) -> Elasticsearch {
    let builder = create_conn_builder(url);
    create(builder)
}

pub fn create(mut connection_builder: ConnectionBuilder) -> Elasticsearch {
    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        connection_builder = connection_builder.proxy(proxy_url);
    }

    let connection = connection_builder.build().unwrap();
    Elasticsearch::new(connection)
}

/// index some documents into a posts index. If the posts index already exists, do nothing.
///
/// As an async fn, this can end up running multiple times concurrently, and indexing documents
/// several times. In this instance, this is fine.
///
/// TODO: This is a temporary measure until https://github.com/elastic/elasticsearch-rs/issues/19 is implemented.
pub async fn index_documents(
    client: &Elasticsearch,
) -> Result<ElasticsearchResponse, ElasticsearchError> {
    let index = "posts";
    let exists_response = client
        .indices()
        .exists(IndicesExistsUrlParts::Index(&[index]))
        .send()
        .await?;

    if exists_response.status_code() == StatusCode::NOT_FOUND {
        let mut body: Vec<JsonBody<_>> = vec![];
        for i in 1..11 {
            let op = json!({"index": {"_id": i}}).into();
            let doc = json!({"title":"Elasticsearch"}).into();
            body.push(op);
            body.push(doc);
        }

        client
            .bulk(BulkUrlParts::Index(index))
            .body(body)
            .refresh(Refresh::WaitFor)
            .send()
            .await
    } else {
        Ok(exists_response)
    }
}
