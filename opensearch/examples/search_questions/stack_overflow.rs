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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A Stack Overflow post
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Post {
    Question(Question),
    Answer(Answer),
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
