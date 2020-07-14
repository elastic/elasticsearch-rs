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
use chrono::{prelude::*, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fs::File, io::Read};
use xml::{reader::XmlEvent, EventReader};

/// A Stack Overflow post
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Post {
    Question(Question),
    Answer(Answer),
}

impl Post {
    pub fn id(&self) -> i32 {
        match self {
            Post::Question(q) => q.id,
            Post::Answer(a) => a.id,
        }
    }
}

/// A Stack Overflow question
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: i32,
    pub parent_id: Value,
    pub creation_date: DateTime<Utc>,
    pub score: i32,
    pub body: String,
    pub owner_user_id: Option<i32>,
    pub owner_display_name: Option<String>,
    pub last_editor_user_id: Option<i32>,
    pub last_edit_date: Option<DateTime<Utc>>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub comment_count: i32,
    pub tags: Vec<String>,
    pub title: String,
    pub title_suggest: Option<Value>,
    pub accepted_answer_id: Option<i32>,
    pub view_count: i32,
    pub last_editor_display_name: Option<String>,
    pub answer_count: i32,
    pub favorite_count: i32,
    pub community_owned_date: Option<DateTime<Utc>>,
}

impl From<Question> for Post {
    fn from(q: Question) -> Self {
        Post::Question(q)
    }
}

/// A Stack Overflow answer
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Answer {
    pub id: i32,
    pub parent_id: Value,
    pub creation_date: DateTime<Utc>,
    pub score: i32,
    pub body: String,
    pub owner_user_id: Option<i32>,
    pub owner_display_name: Option<String>,
    pub last_editor_user_id: Option<i32>,
    pub last_edit_date: Option<DateTime<Utc>>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub comment_count: i32,
}

impl From<Answer> for Post {
    fn from(a: Answer) -> Self {
        Post::Answer(a)
    }
}

pub struct PostsIter {
    reader: EventReader<File>,
    finished: bool,
}

impl PostsIter {
    fn open_xml(path: &str) -> File {
        let mut xml = File::open(path).unwrap();
        // skip the BOM as the xml library doesn't handle this.
        let mut bom = [0; 3];
        xml.read_exact(&mut bom).unwrap();
        xml
    }

    pub fn new(path: &str) -> Self {
        let xml = Self::open_xml(path);
        Self {
            reader: EventReader::new(xml),
            finished: false,
        }
    }
}

impl Iterator for PostsIter {
    type Item = Post;

    fn next(&mut self) -> Option<Post> {
        if self.finished {
            return None;
        }

        fn parse_datetime_utc<S: AsRef<str>>(s: S) -> DateTime<Utc> {
            Utc.datetime_from_str(s.as_ref(), "%Y-%m-%dT%H:%M:%S.%f")
                .unwrap()
        }

        let post = match self.reader.next() {
            Ok(e) => match e {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_ref() {
                    "row" => {
                        let mut a = BTreeMap::new();
                        for attribute in attributes {
                            a.insert(attribute.name.local_name, attribute.value);
                        }

                        let id = a["Id"].parse::<i32>().unwrap();
                        let post_type_id = a["PostTypeId"].parse::<i32>().unwrap();
                        let score = a["Score"].parse::<i32>().unwrap();
                        let body = a["Body"].clone();
                        let creation_date = parse_datetime_utc(a["CreationDate"].as_str());
                        let comment_count = a["CommentCount"].parse::<i32>().unwrap();
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
                            Some(parse_datetime_utc(a["LastEditDate"].as_str()))
                        } else {
                            None
                        };

                        let last_activity_date = if a.contains_key("LastActivityDate") {
                            Some(parse_datetime_utc(a["LastActivityDate"].as_str()))
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
                                view_count: a["ViewCount"].parse::<i32>().unwrap(),
                                last_editor_display_name: a
                                    .get("LastEditorDisplayName")
                                    .map(|s| s.clone()),
                                answer_count: a["AnswerCount"].parse::<i32>().unwrap(),
                                favorite_count: a
                                    .get("FavoriteCount")
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .unwrap_or_else(|| 0),
                                community_owned_date: a
                                    .get("CommunityOwnedDate")
                                    .map(|s| parse_datetime_utc(s)),
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

                        Some(post)
                    }
                    _ => self.next(),
                },
                XmlEvent::EndDocument => {
                    self.finished = true;
                    None
                }
                _ => self.next(),
            },
            Err(e) => {
                self.finished = true;
                println!("{:?}", e);
                None
            }
        };

        post
    }
}
