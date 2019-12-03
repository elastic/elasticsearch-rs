use elasticsearch::{ConnectionBuilder, Elasticsearch, DEFAULT_ADDRESS, Credentials, ElasticsearchResponse, ElasticsearchError, indices::IndicesExistsUrlParts};

use sysinfo::SystemExt;
use url::Url;
use reqwest::StatusCode;
use http::header::CONTENT_TYPE;
use http::HeaderValue;
use reqwest::header::{ACCEPT, HeaderMap};

fn cluster_addr() -> String {
    match std::env::var("ES_TEST_SERVER") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

fn running_proxy() -> bool {
    // check if the Fiddler process is running, and hook it up as a proxy if so.
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

fn create_reqwest_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let mut builder = reqwest::ClientBuilder::new().default_headers(headers);

    if running_proxy() {
        builder = builder.proxy(reqwest::Proxy::http("http://localhost:8888").unwrap());
    }

    builder.build().unwrap()
}

/// index some documents into a posts index. If the posts index already exists, do nothing.
///
/// As an async fn, this can end up running multiple times concurrently, and indexing documents
/// several times. In this instance, this is fine.
///
/// TODO: This is a temporary measure until https://github.com/elastic/elasticsearch-rs/issues/19 is implemented.
pub async fn index_documents(client: &Elasticsearch) -> Result<ElasticsearchResponse, ElasticsearchError> {
    let index = "posts";
    let exists_response = client.indices()
        .exists(IndicesExistsUrlParts::Index(&[index]))
        .send()
        .await?;

    if exists_response.status_code() == StatusCode::NOT_FOUND {
        let mut body: Vec<u8> = vec![];
        for i in 1..11 {
            let mut op_doc =
                format!("{{\"index\":{{\"_id\":{}}}}}\n{{\"title\":\"Elasticsearch\"}}\n", i).as_bytes().to_vec();
            body.append(&mut op_doc);
        }

        let bulk_endpoint = format!("{}/{}/_bulk?refresh=wait_for", index, cluster_addr());
        let reqwest_client = create_reqwest_client();
        let response = reqwest_client.post(&bulk_endpoint)
            .body(body)
            .send()
            .await;

        match response {
            Ok(r) => Ok(ElasticsearchResponse::new(r)),
            Err(e) => Err(ElasticsearchError::HttpError(e)),
        }
    } else {
        Ok(exists_response)
    }
}