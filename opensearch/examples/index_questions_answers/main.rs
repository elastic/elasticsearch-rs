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
#[macro_use]
extern crate serde_json;

use clap::{App, Arg};
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::cert::CertificateValidation;
use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    indices::{
        IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts, IndicesPutSettingsParts,
    },
    BulkOperation, BulkParts, Elasticsearch, Error, DEFAULT_ADDRESS,
};
use serde_json::Value;
use sysinfo::SystemExt;
use url::Url;

mod stack_overflow;
use http::StatusCode;
use stack_overflow::*;
use std::time::Instant;

static POSTS_INDEX: &'static str = "posts";

/// Reads questions and answers from the Stack Overflow Data Dump XML file and indexes
/// them into Elasticsearch using the bulk API. An index with explicit mapping is created
/// for search in other examples.
///
/// Stack Overflow data is licensed under cc-by-sa 4.0.
// TODO: Concurrent bulk requests
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("index_questions_answers")
        .about(
            "indexes Stack Overflow questions and answers into Elasticsearch with the Rust client",
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .value_name("PATH")
                .help("The path to the Posts.xml file containing questions and answers. Can be obtained from https://archive.org/download/stackexchange/stackoverflow.com-Posts.7z (large file)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("limit")
                .short("l")
                .long("limit")
                .value_name("LIMIT")
                .help("The number of questions and answers from Posts.xml to index")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("SIZE")
                .help("The number of documents in each bulk request")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .help("Whether to delete the index before indexing")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let path = matches.value_of("path").expect("missing 'path' argument");
    let limit = match matches.value_of("limit") {
        Some(l) => Some(l.parse::<usize>()?),
        _ => None,
    };
    let size = match matches.value_of("size") {
        Some(l) => l.parse::<usize>()?,
        _ => 1000,
    };

    let delete = matches.is_present("delete");
    let client = create_client()?;

    create_index_if_not_exists(&client, delete).await?;
    set_refresh_interval(&client, json!("-1")).await?;

    let mut posts_iter = PostsIter::new(path);
    let mut total = 0;
    let mut posts = Vec::with_capacity(size);
    let now = Instant::now();

    while let Some(post) = posts_iter.next() {
        total += 1;
        posts.push(post);
        if total % size == 0 {
            index_posts(&client, &posts).await?;
            let duration = now.elapsed();
            let secs = duration.as_secs_f64();

            let taken = if secs >= 60f64 {
                format!("{}m", secs / 60f64)
            } else {
                format!("{:?}", duration)
            };

            println!("Indexed total {} posts in {}", total, taken);
            posts.clear();
        }

        if let Some(l) = limit {
            if total >= l {
                break;
            }
        }
    }

    if !posts.is_empty() {
        index_posts(&client, &posts).await?;
        posts.clear();
    }

    set_refresh_interval(&client, json!(null)).await?;
    Ok(())
}

async fn set_refresh_interval(client: &Elasticsearch, interval: Value) -> Result<(), Error> {
    let response = client
        .indices()
        .put_settings(IndicesPutSettingsParts::Index(&[POSTS_INDEX]))
        .body(json!({
            "index" : {
                "refresh_interval" : interval
            }
        }))
        .send()
        .await?;

    if !response.status_code().is_success() {
        println!("Failed to update refresh interval");
    }

    Ok(())
}

async fn index_posts(client: &Elasticsearch, posts: &[Post]) -> Result<(), Error> {
    let body: Vec<BulkOperation<_>> = posts
        .iter()
        .map(|p| {
            let id = p.id().to_string();
            BulkOperation::index(p).id(&id).routing(&id).into()
        })
        .collect();

    let response = client
        .bulk(BulkParts::Index(POSTS_INDEX))
        .body(body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        // TODO: retry failures
        println!("Errors whilst indexing. Failures: {}", failed.len());
    }

    Ok(())
}

async fn create_index_if_not_exists(client: &Elasticsearch, delete: bool) -> Result<(), Error> {
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[POSTS_INDEX]))
        .send()
        .await?;

    if exists.status_code().is_success() && delete {
        let delete = client
            .indices()
            .delete(IndicesDeleteParts::Index(&[POSTS_INDEX]))
            .send()
            .await?;

        if !delete.status_code().is_success() {
            println!("Problem deleting index: {}", delete.text().await?);
        }
    }

    if exists.status_code() == StatusCode::NOT_FOUND || delete {
        let response = client
            .indices()
            .create(IndicesCreateParts::Index(POSTS_INDEX))
            .body(json!(
                {
                  "mappings": {
                    "properties": {
                      "type": {
                        "type": "keyword"
                      },
                      "id": {
                        "type": "integer"
                      },
                      "parent_id": {
                        "relations": {
                          "question": "answer"
                        },
                        "type": "join"
                      },
                      "creation_date": {
                        "type": "date"
                      },
                      "score": {
                        "type": "integer"
                      },
                      "body": {
                        "analyzer": "html",
                        "search_analyzer": "expand",
                        "type": "text"
                      },
                      "owner_user_id": {
                        "type": "integer"
                      },
                      "owner_display_name": {
                        "type": "keyword"
                      },
                      "last_editor_user_id": {
                        "type": "integer"
                      },
                      "last_edit_date": {
                        "type": "date"
                      },
                      "last_activity_date": {
                        "type": "date"
                      },
                      "comment_count": {
                        "type": "integer"
                      },
                      "title": {
                        "analyzer": "expand",
                        "norms": false,
                        "fields": {
                          "raw": {
                            "type": "keyword"
                          }
                        },
                        "type": "text"
                      },
                      "title_suggest": {
                        "type": "completion"
                      },
                      "accepted_answer_id": {
                        "type": "integer"
                      },
                      "view_count": {
                        "type": "integer"
                      },
                      "last_editor_display_name": {
                        "type": "keyword"
                      },
                      "tags": {
                        "type": "keyword"
                      },
                      "answer_count": {
                        "type": "integer"
                      },
                      "favorite_count": {
                        "type": "integer"
                      },
                      "community_owned_date": {
                        "type": "date"
                      }
                    },
                    "_routing": {
                      "required": true
                    },
                    "_source": {
                      "excludes": ["title_suggest"]
                    }
                  },
                  "settings": {
                    "index.number_of_shards": 3,
                    "index.number_of_replicas": 0,
                    "analysis": {
                      "analyzer": {
                        "html": {
                          "char_filter": ["html_strip", "programming_language"],
                          "filter": ["lowercase", "stop"],
                          "tokenizer": "standard",
                          "type": "custom"
                        },
                        "expand": {
                          "char_filter": ["programming_language"],
                          "filter": ["lowercase", "stop"],
                          "tokenizer": "standard",
                          "type": "custom"
                        }
                      },
                      "char_filter": {
                        "programming_language": {
                          "mappings": [
                            "c# => csharp", "C# => csharp",
                            "f# => fsharp", "F# => fsharp",
                            "m# => msharp", "M# => msharp",
                            "j# => jsharp", "J# => jsharp",
                            "s# => ssharp", "S# => ssharp",
                            "a# => asharp", "A# => asharp",
                            "k# => ksharp", "K# => ksharp",
                            "t# => tsharp", "T# => tsharp",
                            "g++ => gplusplus", "G++ => gplusplus",
                            "m++ => mplusplus", "M++ => mplusplus",
                            "c++ => cplusplus", "C++ => cplusplus",
                            "s++ => splusplus", "S++ => splusplus",
                            "a++ => aplusplus", "A++ => aplusplus",
                            "d++ => dplusplus", "D++ => dplusplus"
                          ],
                          "type": "mapping"
                        }
                      }
                    }
                  }
                }
            ))
            .send()
            .await?;

        if !response.status_code().is_success() {
            println!("Error while creating index");
        }
    }

    Ok(())
}

fn create_client() -> Result<Elasticsearch, Error> {
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
        Some(c) => {
            builder = builder.auth(c);

            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            {
                builder = builder.cert_validation(CertificateValidation::None);
            }

            builder
        }
        None => builder,
    };

    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}
