use elasticsearch::{ConnectionBuilder, Elasticsearch, DEFAULT_ADDRESS};

use sysinfo::SystemExt;
use url::Url;

pub fn create_conn_builder(addr: &str) -> ConnectionBuilder {
    let url = Url::parse(addr).unwrap();
    ConnectionBuilder::new(url)
}

pub fn create_default() -> Elasticsearch {
    create_for_url(DEFAULT_ADDRESS)
}

pub fn create_for_url(url: &str) -> Elasticsearch {
    let builder = create_conn_builder(url);
    create(builder)
}

pub fn create(mut connection_builder: ConnectionBuilder) -> Elasticsearch {
    // check if the Fiddler process is running, and hook it up as a proxy if so.
    let system = sysinfo::System::new();
    if !system.get_process_by_name("Fiddler").is_empty() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        connection_builder = connection_builder.proxy(proxy_url);
    }

    let connection = connection_builder.build().unwrap();
    Elasticsearch::new(connection)
}
