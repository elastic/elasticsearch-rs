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
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Record {
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,
    pub labels: BTreeMap<String, String>,
    pub tags: Vec<String>,
    pub event: Event,
    pub http: Http,
    pub benchmark: Benchmark,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Event {
    pub action: String,
    pub duration: i64,
    pub outcome: String,
    pub dataset: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Http {
    pub response: HttpResponse,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct HttpResponse {
    pub status_code: Option<u16>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Benchmark {
    pub build_id: String,
    pub repetitions: i32,
    pub operations: i32,
    pub runner: Runner,
    pub target: Target,
    pub category: String,
    pub environment: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Runner {
    pub service: Service,
    pub runtime: Runtime,
    pub os: Os,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Target {
    pub service: Service,
    pub os: Os,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Runtime {
    pub name: String,
    pub version: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Service {
    #[serde(rename = "type")]
    pub ty: String,
    pub name: String,
    pub version: String,
    pub git: Git,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Git {
    pub commit: String,
    pub branch: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Os {
    pub family: String,
}
