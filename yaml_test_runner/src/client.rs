use elasticsearch::{
    auth::Credentials,
    http::{
        transport::{
            TransportBuilder,
            SingleNodeConnectionPool
        }
    },
    Elasticsearch, DEFAULT_ADDRESS,
    cert::CertificateValidation
};
use sysinfo::SystemExt;
use url::Url;

fn cluster_addr() -> String {
    match std::env::var("ES_TEST_SERVER") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

fn running_proxy() -> bool {
    let system = sysinfo::System::new();
    !system.get_process_by_name("Fiddler").is_empty()
}

pub fn create() -> Elasticsearch {
    let url = Url::parse(cluster_addr().as_ref()).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url.clone());
    let mut builder = TransportBuilder::new(conn_pool);

    if url.scheme() == "https" {
        builder = builder
            .auth(Credentials::Basic("elastic".into(), "changeme".into()))
            .cert_validation(CertificateValidation::None)
    }

    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build().unwrap();
    Elasticsearch::new(transport)
}

