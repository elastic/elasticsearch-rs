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
// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo run -p api_generator
// -----------------------------------------------
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "The unit in which to display byte values"]
pub enum Bytes {
    #[serde(rename = "b")]
    B,
    #[serde(rename = "k")]
    K,
    #[serde(rename = "kb")]
    Kb,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "mb")]
    Mb,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "t")]
    T,
    #[serde(rename = "tb")]
    Tb,
    #[serde(rename = "p")]
    P,
    #[serde(rename = "pb")]
    Pb,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "What to do when the delete by query hits version conflicts?"]
pub enum Conflicts {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "proceed")]
    Proceed,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "The default operator for query string query (AND or OR)"]
pub enum DefaultOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
pub enum ExpandWildcards {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Group tasks by nodes or parent/child relationships"]
pub enum GroupBy {
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "parents")]
    Parents,
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
pub enum Health {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Specify the level of detail for returned information"]
pub enum Level {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "indices")]
    Indices,
    #[serde(rename = "shards")]
    Shards,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID"]
pub enum OpType {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` (the default) then do nothing with refreshes."]
pub enum Refresh {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "wait_for")]
    WaitFor,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Search operation type"]
pub enum SearchType {
    #[serde(rename = "query_then_fetch")]
    QueryThenFetch,
    #[serde(rename = "dfs_query_then_fetch")]
    DfsQueryThenFetch,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "The multiplier in which to display values"]
pub enum Size {
    #[serde(rename = "")]
    Unspecified,
    #[serde(rename = "k")]
    K,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "t")]
    T,
    #[serde(rename = "p")]
    P,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Specify suggest mode"]
pub enum SuggestMode {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "always")]
    Always,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "The unit in which to display time values"]
pub enum Time {
    #[serde(rename = "d")]
    D,
    #[serde(rename = "h")]
    H,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "micros")]
    Micros,
    #[serde(rename = "nanos")]
    Nanos,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "The type to sample (default: cpu)"]
pub enum Type {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "block")]
    Block,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Specific version type"]
pub enum VersionType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Wait until all currently queued events with the given priority are processed"]
pub enum WaitForEvents {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "urgent")]
    Urgent,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "languid")]
    Languid,
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
#[doc = "Wait until cluster is in a specific state"]
pub enum WaitForStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
