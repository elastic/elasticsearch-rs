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
//! HTTP header names and values, including those specific to Elasticsearch

pub use reqwest::header::*;

/// The default user-agent header value sent by the client
pub static DEFAULT_USER_AGENT: &str = concat!("elasticsearch-rs/", env!("CARGO_PKG_VERSION"));

/// The default content-type header value of `application/json`
pub static DEFAULT_CONTENT_TYPE: &str = "application/json";

/// The default accept header value of `application/json`
pub static DEFAULT_ACCEPT: &str = "application/json";

/// The X-Opaque-Id header name, used to track certain calls, or associate
/// certain tasks with a client that started them.
pub static X_OPAQUE_ID: &str = "x-opaque-id";
