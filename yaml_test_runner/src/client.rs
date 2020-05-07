use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch, DEFAULT_ADDRESS, Error
};
use sysinfo::SystemExt;
use url::Url;
use elasticsearch::indices::{IndicesDeleteParts, IndicesDeleteTemplateParts};
use elasticsearch::params::ExpandWildcards;
use elasticsearch::cat::{CatTemplatesParts, CatSnapshotsParts};
use elasticsearch::snapshot::{SnapshotDeleteParts, SnapshotDeleteRepositoryParts};

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

pub async fn general_oss_teardown(client: &Elasticsearch) -> Result<(), Error> {
    let _delete_response = client.indices()
        .delete(IndicesDeleteParts::Index(&["*"]))
        .expand_wildcards(&[ExpandWildcards::Open, ExpandWildcards::Closed, ExpandWildcards::Hidden])
        .send()
        .await?;

    let cat_template_response = client.cat()
        .templates(CatTemplatesParts::Name("*"))
        .h(&["name"])
        .send()
        .await?
        .text()
        .await?;

    let all_templates: Vec<&str> = cat_template_response
        .split('\n')
        .filter(|s| !s.is_empty() && !s.starts_with('.') && s != &"security-audit-log")
        .collect();

    for template in all_templates {
        let _delete_template_response = client.indices()
            .delete_template(IndicesDeleteTemplateParts::Name(&template))
            .send()
            .await?;
    }

    let cat_snapshot_response = client.cat()
        .snapshots(CatSnapshotsParts::None)
        .h(&["id", "repository"])
        .send()
        .await?
        .text()
        .await?;

    let all_snapshots: Vec<(&str, &str)> = cat_snapshot_response
        .split('\n')
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .filter(|s| s.len() == 2)
        .map(|s| (s[0].trim(), s[1].trim()))
        .filter(|(id, repo)| !id.is_empty() && !repo.is_empty())
        .collect();

    for (id, repo) in all_snapshots {
        let _snapshot_response = client.snapshot()
            .delete(SnapshotDeleteParts::RepositorySnapshot(&repo, &id))
            .send()
            .await?;
    }

    let _delete_repo_response = client.snapshot()
        .delete_repository(SnapshotDeleteRepositoryParts::Repository(&["*"]))
        .send()
        .await?;

    Ok(())
}
