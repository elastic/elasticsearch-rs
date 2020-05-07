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
use elasticsearch::watcher::WatcherDeleteWatchParts;
use serde_json::Value;
use std::future::Future;
use elasticsearch::security::{SecurityGetRoleParts, SecurityDeleteRoleParts, SecurityGetUserParts, SecurityDeleteUserParts, SecurityGetPrivilegesParts, SecurityDeletePrivilegesParts};
use elasticsearch::ml::{MlStopDatafeedParts, MlGetDatafeedsParts, MlDeleteDatafeedParts, MlCloseJobParts, MlGetJobsParts, MlDeleteJobParts};
use elasticsearch::ilm::IlmRemovePolicyParts;

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

async fn delete_all_templates(client: &Elasticsearch) -> Result<(), Error> {
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

    Ok(())
}

/// general teardown step for an OSS yaml test
pub async fn general_oss_teardown(client: &Elasticsearch) -> Result<(), Error> {
    let _delete_response = client.indices()
        .delete(IndicesDeleteParts::Index(&["*"]))
        .expand_wildcards(&[ExpandWildcards::Open, ExpandWildcards::Closed, ExpandWildcards::Hidden])
        .send()
        .await?;

    delete_all_templates(client).await?;

    let cat_snapshot_response = client.cat()
        .snapshots(CatSnapshotsParts::None)
        .h(&["id", "repository"])
        .send()
        .await?;

    if cat_snapshot_response.status_code().is_success() {
        let cat_snapshot_text = cat_snapshot_response
            .text()
            .await?;

        let all_snapshots: Vec<(&str, &str)> = cat_snapshot_text
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
    }

    let _delete_repo_response = client.snapshot()
        .delete_repository(SnapshotDeleteRepositoryParts::Repository(&["*"]))
        .send()
        .await?;

    Ok(())
}

pub async fn general_xpack_teardown(client: &Elasticsearch) -> Result<(), Error> {
    delete_all_templates(client).await?;

    let _delete_watch_response = client.watcher()
        .delete_watch(WatcherDeleteWatchParts::Id("my_watch"))
        .send()
        .await?;

    let roles_response = client.security()
        .get_role(SecurityGetRoleParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in roles_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let _ = client.security()
                    .delete_role(SecurityDeleteRoleParts::Name(k))
                    .send()
                    .await?;
            }
        }
    }

    let users_response = client.security()
        .get_user(SecurityGetUserParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in users_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let _ = client.security()
                    .delete_user(SecurityDeleteUserParts::Username(k))
                    .send()
                    .await?;
            }
        }
    }

    delete_privileges(client).await?;
    stop_and_delete_datafeeds(client).await?;

    let _ = client.ilm()
        .remove_policy(IlmRemovePolicyParts::Index("_all"))
        .send()
        .await?;

    close_and_delete_jobs(client).await?;

    Ok(())
}

async fn delete_privileges(client: &Elasticsearch) -> Result<(), Error> {
    let privileges_response = client.security()
        .get_privileges(SecurityGetPrivilegesParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in privileges_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let _ = client.security()
                    .delete_privileges(SecurityDeletePrivilegesParts::ApplicationName(k, "_all"))
                    .send()
                    .await?;
            }
        }
    }

    Ok(())
}

async fn stop_and_delete_datafeeds(client: &Elasticsearch) -> Result<(), Error> {
    let _stop_data_feed_response = client.ml()
        .stop_datafeed(MlStopDatafeedParts::DatafeedId("_all"))
        .send()
        .await?;

    let get_data_feeds_response = client.ml()
        .get_datafeeds(MlGetDatafeedsParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for feed in get_data_feeds_response["datafeeds"].as_array().unwrap() {
        let id = feed["datafeed_id"].as_str().unwrap();
        let _ = client.ml()
            .delete_datafeed(MlDeleteDatafeedParts::DatafeedId(id))
            .send()
            .await?;
    }

    Ok(())
}

async fn close_and_delete_jobs(client: &Elasticsearch) -> Result<(), Error> {
    let _ = client.ml()
        .close_job(MlCloseJobParts::JobId("_all"))
        .send()
        .await?;

    let get_jobs_response = client.ml()
        .get_jobs(MlGetJobsParts::JobId("_all"))
        .send()
        .await?
        .json::<Value>()
        .await?;

    for job in get_jobs_response["jobs"].as_array().unwrap() {
        let id = job["job_id"].as_str().unwrap();
        let _ = client.ml()
            .delete_job(MlDeleteJobParts::JobId(id))
            .send()
            .await?;
    }

    Ok(())
}
