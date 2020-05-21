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
use elasticsearch::http::{response::Response, Method, StatusCode};
use serde_json::Value;

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
