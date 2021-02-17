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
use elasticsearch::{
    auth::Credentials,
    cat::CatTemplatesParts,
    cert::CertificateValidation,
    cluster::ClusterHealthParts,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Method, StatusCode,
    },
    ilm::IlmRemovePolicyParts,
    indices::{
        IndicesDeleteIndexTemplateParts, IndicesDeleteParts, IndicesDeleteTemplateParts,
        IndicesRefreshParts,
    },
    ml::{
        MlCloseJobParts, MlDeleteDatafeedParts, MlDeleteJobParts, MlGetDatafeedsParts,
        MlGetJobsParts, MlStopDatafeedParts,
    },
    params::{ExpandWildcards, WaitForStatus},
    security::{
        SecurityDeletePrivilegesParts, SecurityDeleteRoleParts, SecurityDeleteUserParts,
        SecurityGetPrivilegesParts, SecurityGetRoleParts, SecurityGetUserParts,
        SecurityPutUserParts,
    },
    snapshot::{SnapshotDeleteParts, SnapshotDeleteRepositoryParts},
    tasks::TasksCancelParts,
    transform::{
        TransformDeleteTransformParts, TransformGetTransformParts, TransformStopTransformParts,
    },
    watcher::WatcherDeleteWatchParts,
    Elasticsearch, Error, DEFAULT_ADDRESS,
};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::ops::Deref;
use sysinfo::SystemExt;
use url::Url;

fn cluster_addr() -> String {
    match std::env::var("ELASTICSEARCH_URL") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Determines if Fiddler.exe proxy process is running
fn running_proxy() -> bool {
    let system = sysinfo::System::new();
    !system.get_process_by_name("Fiddler").is_empty()
}

static GLOBAL_CLIENT: Lazy<Elasticsearch> = Lazy::new(|| {
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

    let transport = builder.build().unwrap();
    Elasticsearch::new(transport)
});

/// Gets the client to use in tests
pub fn get() -> &'static Elasticsearch {
    GLOBAL_CLIENT.deref()
}

/// Reads the response from Elasticsearch, returning the method, status code, text response,
/// and the response parsed from json or yaml
pub async fn read_response(
    response: Response,
) -> Result<(Method, StatusCode, String, Value), failure::Error> {
    let is_json = response.content_type().starts_with("application/json");
    let is_yaml = response.content_type().starts_with("application/yaml");
    let method = response.method();
    let status_code = response.status_code();
    let text = response.text().await?;
    let json = if is_json && !text.is_empty() {
        serde_json::from_str::<Value>(text.as_ref())?
    } else if is_yaml && !text.is_empty() {
        serde_yaml::from_str::<Value>(text.as_ref())?
    } else {
        Value::Null
    };

    Ok((method, status_code, text, json))
}

/// general setup step for an OSS yaml test
pub async fn general_oss_setup() -> Result<(), Error> {
    let client = get();
    delete_data_streams(client).await?;
    delete_indices(client).await?;
    delete_templates(client).await?;
    delete_snapshots(client).await?;

    Ok(())
}

pub async fn delete_snapshots(client: &Elasticsearch) -> Result<(), Error> {
    let cat_repo_response = client
        .cat()
        .repositories()
        .h(&["id"])
        .send()
        .await?
        .error_for_status_code()?
        .text()
        .await?;

    if cat_repo_response.len() > 0 {
        let repositories: Vec<&str> = cat_repo_response.split_terminator('\n').collect();

        // Delete snapshots in each repository
        for repo in repositories {
            let delete_snapshots_response = client
                .snapshot()
                .delete(SnapshotDeleteParts::RepositorySnapshot(repo, "*"))
                .send()
                .await?;

            assert_response_success!(delete_snapshots_response);
        }

        // Delete all snapshot repositories
        let delete_repo_response = client
            .snapshot()
            .delete_repository(SnapshotDeleteRepositoryParts::Repository(&["*"]))
            .send()
            .await?;

        assert_response_success!(delete_repo_response);
    }

    Ok(())
}

/// general setup step for an xpack yaml test
pub async fn general_xpack_setup() -> Result<(), Error> {
    let client = get();

    let _delete_watch_response = client
        .watcher()
        .delete_watch(WatcherDeleteWatchParts::Id("my_watch"))
        .send()
        .await?;

    delete_roles(client).await?;
    delete_users(client).await?;
    delete_privileges(client).await?;
    stop_and_delete_datafeeds(client).await?;

    let response = client
        .ilm()
        .remove_policy(IlmRemovePolicyParts::Index("_all"))
        .send()
        .await?;

    assert_response_success!(response);

    close_and_delete_jobs(client).await?;

    // TODO: stop and delete rollup jobs once implemented in the client

    cancel_tasks(client).await?;
    stop_and_delete_transforms(client).await?;
    wait_for_yellow_status(client).await?;
    delete_data_streams(client).await?;
    delete_indices(client).await?;
    delete_templates(client).await?;

    let response = client
        .security()
        .put_user(SecurityPutUserParts::Username("x_pack_rest_user"))
        .body(json!({
            "password": "x-pack-test-password",
            "roles": ["superuser"]
        }))
        .send()
        .await?;

    assert_response_success!(response);

    let response = client
        .indices()
        .refresh(IndicesRefreshParts::Index(&["_all"]))
        .expand_wildcards(&[
            ExpandWildcards::Open,
            ExpandWildcards::Closed,
            ExpandWildcards::Hidden,
        ])
        .send()
        .await?;

    assert_response_success!(response);

    wait_for_yellow_status(client).await?;

    Ok(())
}

async fn wait_for_yellow_status(client: &Elasticsearch) -> Result<(), Error> {
    let cluster_health = client
        .cluster()
        .health(ClusterHealthParts::None)
        .wait_for_status(WaitForStatus::Yellow)
        .send()
        .await?;

    assert_response_success!(cluster_health);
    Ok(())
}

async fn delete_data_streams(client: &Elasticsearch) -> Result<(), Error> {
    // Hand-crafted request as the indices.delete_data_stream spec doesn't yet have the
    // "expand_wildcards" parameter that is needed to delete ILM data streams
    //
    // Not deleting data streams yields errors like this when trying to delete hidden indices:
    // {
    //   "type":"illegal_argument_exception"
    //   "reason":"index [.ds-ilm-history-5-2021.02.14-000001] is the write index for data
    //             stream [ilm-history-5] and cannot be deleted"
    // }
    //
    // Quoting the docs:
    // You cannot delete the current write index of a data stream. To delete the index,
    // you must roll over the data stream so a new write index is created. You can then use
    // the delete index API to delete the previous write index.
    //
    let delete_response = client
        .transport()
        .send(
            Method::Delete,
            "/_data_stream/*",
            elasticsearch::http::headers::HeaderMap::new(),
            Some(&[("expand_wildcards", "hidden")]),
            None::<()>, // body
            None,       // timeout
        )
        .await?;

    assert_response_success!(delete_response);

    Ok(())
}

async fn delete_indices(client: &Elasticsearch) -> Result<(), Error> {
    let delete_response = client
        .indices()
        .delete(IndicesDeleteParts::Index(&["*"]))
        .expand_wildcards(&[
            ExpandWildcards::Open,
            ExpandWildcards::Closed,
            ExpandWildcards::Hidden,
        ])
        .send()
        .await?;

    assert_response_success!(delete_response);
    Ok(())
}

async fn stop_and_delete_transforms(client: &Elasticsearch) -> Result<(), Error> {
    let transforms_response = client
        .transform()
        .get_transform(TransformGetTransformParts::TransformId("_all"))
        .send()
        .await?
        .json::<Value>()
        .await?;

    for transform in transforms_response["transforms"].as_array().unwrap() {
        let id = transform["id"].as_str().unwrap();
        let response = client
            .transform()
            .stop_transform(TransformStopTransformParts::TransformId(id))
            .send()
            .await?;

        assert_response_success!(response);

        let response = client
            .transform()
            .delete_transform(TransformDeleteTransformParts::TransformId(id))
            .send()
            .await?;

        assert_response_success!(response);
    }

    Ok(())
}

async fn cancel_tasks(client: &Elasticsearch) -> Result<(), Error> {
    let rollup_response = client.tasks().list().send().await?.json::<Value>().await?;

    for (_node_id, nodes) in rollup_response["nodes"].as_object().unwrap() {
        for (task_id, task) in nodes["tasks"].as_object().unwrap() {
            if let Some(b) = task["cancellable"].as_bool() {
                if b {
                    let response = client
                        .tasks()
                        .cancel(TasksCancelParts::TaskId(task_id))
                        .send()
                        .await?;

                    assert_response_success!(response);
                }
            }
        }
    }

    Ok(())
}

async fn delete_templates(client: &Elasticsearch) -> Result<(), Error> {
    // There are "legacy templates and "new templates"

    let cat_template_response = client
        .cat()
        .templates(CatTemplatesParts::Name("*"))
        .h(&["name"])
        .send()
        .await?
        .text()
        .await?;

    let all_templates: Vec<&str> = cat_template_response
        .split_terminator('\n')
        .filter(|s| !s.starts_with('.') && s != &"security-audit-log")
        .collect();

    for template in all_templates {
        if template == "ilm-history" {
            // We may need to extend this to mimic ESRestTestCase.isXPackTemplate() from the ES
            // test harness
            continue;
        }

        let mut delete_template_response = client
            .indices()
            .delete_index_template(IndicesDeleteIndexTemplateParts::Name(&template))
            .send()
            .await?;

        if delete_template_response.status_code().as_u16() == 404 {
            // Certainly an old-style template
            delete_template_response = client
                .indices()
                .delete_template(IndicesDeleteTemplateParts::Name(&template))
                .send()
                .await?;
        }
        assert_response_success!(delete_template_response);
    }

    Ok(())
}

async fn delete_users(client: &Elasticsearch) -> Result<(), Error> {
    let users_response = client
        .security()
        .get_user(SecurityGetUserParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in users_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_user(SecurityDeleteUserParts::Username(k))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}

async fn delete_roles(client: &Elasticsearch) -> Result<(), Error> {
    let roles_response = client
        .security()
        .get_role(SecurityGetRoleParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in roles_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_role(SecurityDeleteRoleParts::Name(k))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}

async fn delete_privileges(client: &Elasticsearch) -> Result<(), Error> {
    let privileges_response = client
        .security()
        .get_privileges(SecurityGetPrivilegesParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in privileges_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_privileges(SecurityDeletePrivilegesParts::ApplicationName(k, "_all"))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}

async fn stop_and_delete_datafeeds(client: &Elasticsearch) -> Result<(), Error> {
    let stop_data_feed_response = client
        .ml()
        .stop_datafeed(MlStopDatafeedParts::DatafeedId("_all"))
        .send()
        .await?;

    assert_response_success!(stop_data_feed_response);

    let get_data_feeds_response = client
        .ml()
        .get_datafeeds(MlGetDatafeedsParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for feed in get_data_feeds_response["datafeeds"].as_array().unwrap() {
        let id = feed["datafeed_id"].as_str().unwrap();
        let _ = client
            .ml()
            .delete_datafeed(MlDeleteDatafeedParts::DatafeedId(id))
            .send()
            .await?;
    }

    Ok(())
}

async fn close_and_delete_jobs(client: &Elasticsearch) -> Result<(), Error> {
    let response = client
        .ml()
        .close_job(MlCloseJobParts::JobId("_all"))
        .send()
        .await?;

    assert_response_success!(response);

    let get_jobs_response = client
        .ml()
        .get_jobs(MlGetJobsParts::JobId("_all"))
        .send()
        .await?
        .json::<Value>()
        .await?;

    for job in get_jobs_response["jobs"].as_array().unwrap() {
        let id = job["job_id"].as_str().unwrap();
        let response = client
            .ml()
            .delete_job(MlDeleteJobParts::JobId(id))
            .send()
            .await?;

        assert_response_success!(response);
    }

    Ok(())
}
