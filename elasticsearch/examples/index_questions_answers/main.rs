#[macro_use]
extern crate serde_json;

use chrono::prelude::*;
use chrono::ParseResult;
use clap::{App, Arg};
use elasticsearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    indices::{IndicesCreateParts, IndicesExistsParts},
    BulkOperation, BulkParts, Elasticsearch, Error, DEFAULT_ADDRESS,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::str;
use sysinfo::SystemExt;
use url::Url;
use xml::reader::{EventReader, XmlEvent};

mod stack_overflow;
use elasticsearch::indices::IndicesDeleteParts;
use http::StatusCode;
use stack_overflow::*;

/// Reads questions and answers from the Stack Overflow Data Dump XML file and indexes
/// them into Elasticsearch using the bulk API. An index with explicit mapping is created
/// for search in other examples.
///
/// Stack Overflow data is licensed under cc-by-sa 4.0.
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
        Some(l) => Some(l.parse::<i32>()?),
        _ => None,
    };
    let size = match matches.value_of("size") {
        Some(l) => l.parse::<usize>()?,
        _ => 1000,
    };

    let delete = matches.is_present("delete");

    let client = create_client()?;

    create_index_if_not_exists(&client, delete).await?;

    let xml = open_xml(path)?;
    let reader = EventReader::new(xml);
    let mut count = 0;
    let mut posts = Vec::with_capacity(size);

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                match name.local_name.as_ref() {
                    "row" => {
                        count += 1;

                        let mut a = BTreeMap::new();
                        for attribute in attributes {
                            a.insert(attribute.name.local_name, attribute.value);
                        }

                        let id = a["Id"].parse::<i32>()?;
                        let post_type_id = a["PostTypeId"].parse::<i32>()?;
                        let score = a["Score"].parse::<i32>()?;
                        let body = a["Body"].clone();
                        let creation_date = parse_datetime_utc(a["CreationDate"].as_str())?;
                        let comment_count = a["CommentCount"].parse::<i32>()?;
                        let owner_user_id = if a.contains_key("OwnerUserId") {
                            a["OwnerUserId"].parse::<i32>().ok()
                        } else {
                            None
                        };

                        let owner_display_name = a.get("OwnerDisplayName").map(|s| s.clone());
                        let last_editor_user_id = if a.contains_key("LastEditorUserId") {
                            a["LastEditorUserId"].parse::<i32>().ok()
                        } else {
                            None
                        };

                        let last_edit_date = if a.contains_key("LastEditDate") {
                            Some(parse_datetime_utc(a["LastEditDate"].as_str())?)
                        } else {
                            None
                        };

                        let last_activity_date = if a.contains_key("LastActivityDate") {
                            Some(parse_datetime_utc(a["LastActivityDate"].as_str())?)
                        } else {
                            None
                        };

                        let post: Post = if post_type_id == 1 {
                            let title = a["Title"].clone();
                            let title_suggest = {
                                let weight = if score < 0 { 0 } else { score };
                                json!({
                                    "input": [title],
                                    "weight": weight
                                })
                            };

                            Question {
                                id,
                                parent_id: json!("question"),
                                creation_date,
                                score,
                                body,
                                owner_user_id,
                                owner_display_name,
                                last_editor_user_id,
                                last_edit_date,
                                last_activity_date,
                                comment_count,
                                tags: a
                                    .get("Tags")
                                    .map(|t| {
                                        t.replace(">", "")
                                            .split('<')
                                            .filter(|s| !s.is_empty())
                                            .map(|s| s.to_string())
                                            .collect()
                                    })
                                    .unwrap_or_else(|| vec![]),
                                title,
                                title_suggest: Some(title_suggest),
                                accepted_answer_id: None,
                                view_count: a["ViewCount"].parse::<i32>()?,
                                last_editor_display_name: a
                                    .get("LastEditorDisplayName")
                                    .map(|s| s.clone()),
                                answer_count: a["AnswerCount"].parse::<i32>()?,
                                favorite_count: a
                                    .get("FavoriteCount")
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .unwrap_or_else(|| 0),
                                community_owned_date: a
                                    .get("CommunityOwnedDate")
                                    .map(|s| parse_datetime_utc(s).unwrap()),
                            }
                            .into()
                        } else {
                            Answer {
                                id,
                                body,
                                comment_count,
                                score,
                                creation_date,
                                last_activity_date,
                                last_edit_date,
                                last_editor_user_id,
                                owner_display_name,
                                parent_id: json!({
                                    "parent": a["ParentId"].clone(),
                                    "name": "answer"
                                }),
                                owner_user_id,
                            }
                            .into()
                        };

                        posts.push(post);
                        if count % size == 0 {
                            println!(
                                "Indexing {} posts. count: {}, size: {}",
                                posts.len(),
                                count,
                                size
                            );
                            index_posts(&client, &posts).await?;
                            posts.clear();
                        }
                    }
                    _ => {}
                }

                if let Some(l) = limit {
                    if count as i32 > l {
                        break;
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }

        if !posts.is_empty() {
            index_posts(&client, &posts).await?;
            posts.clear();
        }
    }

    Ok(())
}

fn open_xml(path: &str) -> std::io::Result<File> {
    let mut xml = File::open(path)?;
    // skip the BOM as the xml library doesn't handle this.
    let mut bom = [0; 3];
    xml.read_exact(&mut bom)?;
    Ok(xml)
}

async fn index_posts(client: &Elasticsearch, posts: &[Post]) -> Result<(), Error> {
    let body: Vec<BulkOperation<_>> = posts
        .iter()
        .map(|p| {
            let id = p.id().to_string();
            BulkOperation::index(&id, p).routing(&id).into()
        })
        .collect();

    let response = client
        .bulk(BulkParts::Index("posts"))
        .body(body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if json["errors"].as_bool().unwrap() {
        // TODO collect up the errors.
        println!("Errors whilst indexing");
    }

    Ok(())
}

async fn create_index_if_not_exists(client: &Elasticsearch, delete: bool) -> Result<(), Error> {
    let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&["posts"]))
        .send()
        .await?;

    if exists.status_code().is_success() && delete {
        let delete = client
            .indices()
            .delete(IndicesDeleteParts::Index(&["posts"]))
            .send()
            .await?;

        if !delete.status_code().is_success() {
            println!("Problem deleting index: {}", delete.text().await?);
        }
    }

    if exists.status_code() == StatusCode::NOT_FOUND || delete {
        let response = client
            .indices()
            .create(IndicesCreateParts::Index("posts"))
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

fn parse_datetime_utc<S: AsRef<str>>(s: S) -> ParseResult<DateTime<Utc>> {
    Utc.datetime_from_str(s.as_ref(), "%Y-%m-%dT%H:%M:%S.%f")
}

fn create_client() -> Result<Elasticsearch, Error> {
    fn cluster_addr() -> String {
        match std::env::var("ES_TEST_SERVER") {
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

    let transport = builder.build()?;
    Ok(Elasticsearch::new(transport))
}
