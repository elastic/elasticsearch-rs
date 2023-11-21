/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
    },
    indices::IndicesExistsParts,
    params::Refresh,
    BulkOperation, BulkParts, Elasticsearch, Error, DEFAULT_ADDRESS,
};
use reqwest::StatusCode;
use serde_json::json;
use sysinfo::{RefreshKind, System, SystemExt};
use url::Url;

/// Gets the address to the Elasticsearch instance from environment variables
/// and assumes an instance running locally on the default port otherwise
pub fn cluster_addr() -> String {
    match std::env::var("ELASTICSEARCH_URL") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Checks if Fiddler proxy process is running
fn running_proxy() -> bool {
    let system = System::new_with_specifics(RefreshKind::new().with_processes());
    !system.get_process_by_name("Fiddler").is_empty()
}

pub fn create_default_builder() -> TransportBuilder {
    create_builder(cluster_addr().as_str())
}

pub fn create_builder(addr: &str) -> TransportBuilder {
    let mut url = Url::parse(addr).unwrap();

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

    let conn_pool = SingleNodeConnectionPool::new(url.clone());
    let mut builder = TransportBuilder::new(conn_pool);

    if let Some(c) = credentials {
        builder = builder.auth(c);
    }

    // assume if we're running with HTTPS then disable
    // certificate validation - we'll change this for tests that need to.
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    {
        if url.scheme() == "https" {
            builder = builder.cert_validation(CertificateValidation::None);
        }
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
        for i in 1..=10 {
            let op = BulkOperation::index(json!({"title":"Elasticsearch"}))
                .id(i.to_string())
                .into();
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
