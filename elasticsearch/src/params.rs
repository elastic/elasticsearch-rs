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
//! API parameters

use core::fmt;
use serde::{de, de::Visitor, Deserializer, Serializer};

// GENERATED-BEGIN:spec-params
// Generated code - do not edit until the next GENERATED-END marker

use serde::{Deserialize, Serialize};
#[doc = "The block to add (one of read, write, read_only or metadata)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Block {
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read_only")]
    ReadOnly,
    #[serde(rename = "write")]
    Write,
}
impl ::std::fmt::Display for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Block::Metadata => write!(f, "metadata"),
            Block::Read => write!(f, "read"),
            Block::ReadOnly => write!(f, "read_only"),
            Block::Write => write!(f, "write"),
        }
    }
}
#[doc = "The unit in which to display byte values"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Bytes {
    #[serde(rename = "b")]
    B,
    #[serde(rename = "kb")]
    Kb,
    #[serde(rename = "mb")]
    Mb,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "tb")]
    Tb,
    #[serde(rename = "pb")]
    Pb,
}
impl ::std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Bytes::B => write!(f, "b"),
            Bytes::Kb => write!(f, "kb"),
            Bytes::Mb => write!(f, "mb"),
            Bytes::Gb => write!(f, "gb"),
            Bytes::Tb => write!(f, "tb"),
            Bytes::Pb => write!(f, "pb"),
        }
    }
}
#[doc = "What to do when the update by query hits version conflicts?"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Conflicts {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "proceed")]
    Proceed,
}
impl ::std::fmt::Display for Conflicts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Conflicts::Abort => write!(f, "abort"),
            Conflicts::Proceed => write!(f, "proceed"),
        }
    }
}
#[doc = "The default operator for query string query (AND or OR)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum DefaultOperator {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
impl ::std::fmt::Display for DefaultOperator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DefaultOperator::And => write!(f, "and"),
            DefaultOperator::Or => write!(f, "or"),
        }
    }
}
#[doc = "The mode of compatibility with ECS compliant Grok patterns.\nUse this parameter to specify whether to use ECS Grok patterns instead of legacy ones when the structure finder creates a Grok pattern.\nThis setting primarily has an impact when a whole message Grok pattern such as `%{CATALINALOG}` matches the input.\nIf the structure finder identifies a common structure but has no idea of the meaning then generic field names such as `path`, `ipaddress`, `field1`, and `field2` are used in the `grok_pattern` output.\nThe intention in that situation is that a user who knows the meanings will rename the fields before using them."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum EcsCompatibility {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "v1")]
    V1,
}
impl ::std::fmt::Display for EcsCompatibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EcsCompatibility::Disabled => write!(f, "disabled"),
            EcsCompatibility::V1 => write!(f, "v1"),
        }
    }
}
#[doc = "The format for the response.\nYou can also specify a format using the `Accept` HTTP header.\nIf you specify both this parameter and the `Accept` HTTP header, this parameter takes precedence."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Format {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "tsv")]
    Tsv,
    #[serde(rename = "txt")]
    Txt,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "cbor")]
    Cbor,
    #[serde(rename = "smile")]
    Smile,
}
impl ::std::fmt::Display for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Format::Csv => write!(f, "csv"),
            Format::Json => write!(f, "json"),
            Format::Tsv => write!(f, "tsv"),
            Format::Txt => write!(f, "txt"),
            Format::Yaml => write!(f, "yaml"),
            Format::Cbor => write!(f, "cbor"),
            Format::Smile => write!(f, "smile"),
        }
    }
}
#[doc = "Aggregation used to create a grid for `field`."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum GridAgg {
    #[serde(rename = "geotile")]
    Geotile,
    #[serde(rename = "geohex")]
    Geohex,
}
impl ::std::fmt::Display for GridAgg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GridAgg::Geotile => write!(f, "geotile"),
            GridAgg::Geohex => write!(f, "geohex"),
        }
    }
}
#[doc = "Determines the geometry type for features in the aggs layer."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum GridType {
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "centroid")]
    Centroid,
}
impl ::std::fmt::Display for GridType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GridType::Grid => write!(f, "grid"),
            GridType::Point => write!(f, "point"),
            GridType::Centroid => write!(f, "centroid"),
        }
    }
}
#[doc = "Group tasks by nodes or parent/child relationships"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum GroupBy {
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "parents")]
    Parents,
    #[serde(rename = "none")]
    None,
}
#[cfg(feature = "experimental-apis")]
impl ::std::fmt::Display for GroupBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GroupBy::Nodes => write!(f, "nodes"),
            GroupBy::Parents => write!(f, "parents"),
            GroupBy::None => write!(f, "none"),
        }
    }
}
#[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Health {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::fmt::Display for Health {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Health::Green => write!(f, "green"),
            Health::Yellow => write!(f, "yellow"),
            Health::Red => write!(f, "red"),
            Health::Unknown => write!(f, "unknown"),
            Health::Unavailable => write!(f, "unavailable"),
        }
    }
}
#[doc = "A comma delimited string of optional fields to include in the response\nbody."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Include {
    #[serde(rename = "definition")]
    Definition,
    #[serde(rename = "feature_importance_baseline")]
    FeatureImportanceBaseline,
    #[serde(rename = "hyperparameters")]
    Hyperparameters,
    #[serde(rename = "total_feature_importance")]
    TotalFeatureImportance,
    #[serde(rename = "definition_status")]
    DefinitionStatus,
}
impl ::std::fmt::Display for Include {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Include::Definition => write!(f, "definition"),
            Include::FeatureImportanceBaseline => write!(f, "feature_importance_baseline"),
            Include::Hyperparameters => write!(f, "hyperparameters"),
            Include::TotalFeatureImportance => write!(f, "total_feature_importance"),
            Include::DefinitionStatus => write!(f, "definition_status"),
        }
    }
}
#[doc = "Return indices stats aggregated at index, node or shard level"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Level {
    #[serde(rename = "indices")]
    Indices,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "shards")]
    Shards,
}
impl ::std::fmt::Display for Level {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Level::Indices => write!(f, "indices"),
            Level::Node => write!(f, "node"),
            Level::Shards => write!(f, "shards"),
        }
    }
}
#[doc = "The mapping merge type if mapping overrides are being provided in mapping_addition.\nThe allowed values are one of index or template.\nThe index option merges mappings the way they would be merged into an existing index.\nThe template option merges mappings the way they would be merged into a template."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum MergeType {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "template")]
    Template,
}
#[cfg(feature = "experimental-apis")]
impl ::std::fmt::Display for MergeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MergeType::Index => write!(f, "index"),
            MergeType::Template => write!(f, "template"),
        }
    }
}
#[doc = "REST method to check"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}
#[cfg(feature = "experimental-apis")]
impl ::std::fmt::Display for Method {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
        }
    }
}
#[doc = "Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum OpType {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
}
impl ::std::fmt::Display for OpType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OpType::Index => write!(f, "index"),
            OpType::Create => write!(f, "create"),
        }
    }
}
#[doc = "Sort order"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for Order {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}
#[doc = "The deployment priority."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Priority {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
}
impl ::std::fmt::Display for Priority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Priority::Normal => write!(f, "normal"),
            Priority::Low => write!(f, "low"),
        }
    }
}
#[doc = "If `true` then refresh the affected shards to make this operation visible to search, if `wait_for` (the default) then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Refresh {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "wait_for")]
    WaitFor,
}
impl ::std::fmt::Display for Refresh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Refresh::True => write!(f, "true"),
            Refresh::False => write!(f, "false"),
            Refresh::WaitFor => write!(f, "wait_for"),
        }
    }
}
#[doc = "Search operation type"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum SearchType {
    #[serde(rename = "query_then_fetch")]
    QueryThenFetch,
    #[serde(rename = "dfs_query_then_fetch")]
    DfsQueryThenFetch,
}
impl ::std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SearchType::QueryThenFetch => write!(f, "query_then_fetch"),
            SearchType::DfsQueryThenFetch => write!(f, "dfs_query_then_fetch"),
        }
    }
}
#[doc = "The sort order for 'cpu' type (default: total)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Sort {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "gpu")]
    Gpu,
    #[serde(rename = "mem")]
    Mem,
}
impl ::std::fmt::Display for Sort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Sort::Cpu => write!(f, "cpu"),
            Sort::Wait => write!(f, "wait"),
            Sort::Block => write!(f, "block"),
            Sort::Gpu => write!(f, "gpu"),
            Sort::Mem => write!(f, "mem"),
        }
    }
}
#[doc = "A sync job status to fetch connector sync jobs for"]
#[doc = "&nbsp;\n# Optional, beta\nThis requires the `beta-apis` feature. On track to become stable but breaking changes can\nhappen in minor versions.\n        "]
#[cfg(feature = "beta-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Status {
    #[serde(rename = "canceling")]
    Canceling,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "suspended")]
    Suspended,
}
#[cfg(feature = "beta-apis")]
impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Status::Canceling => write!(f, "canceling"),
            Status::Canceled => write!(f, "canceled"),
            Status::Completed => write!(f, "completed"),
            Status::Error => write!(f, "error"),
            Status::InProgress => write!(f, "in_progress"),
            Status::Pending => write!(f, "pending"),
            Status::Suspended => write!(f, "suspended"),
        }
    }
}
#[doc = "Specify suggest mode"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum SuggestMode {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "always")]
    Always,
}
impl ::std::fmt::Display for SuggestMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SuggestMode::Missing => write!(f, "missing"),
            SuggestMode::Popular => write!(f, "popular"),
            SuggestMode::Always => write!(f, "always"),
        }
    }
}
#[doc = "The task type"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum TaskType {
    #[serde(rename = "text_embedding")]
    TextEmbedding,
    #[serde(rename = "sparse_embedding")]
    SparseEmbedding,
    #[serde(rename = "rerank")]
    Rerank,
    #[serde(rename = "completion")]
    Completion,
}
impl ::std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TaskType::TextEmbedding => write!(f, "text_embedding"),
            TaskType::SparseEmbedding => write!(f, "sparse_embedding"),
            TaskType::Rerank => write!(f, "rerank"),
            TaskType::Completion => write!(f, "completion"),
        }
    }
}
#[doc = "The unit in which to display time values"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
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
impl ::std::fmt::Display for Time {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Time::D => write!(f, "d"),
            Time::H => write!(f, "h"),
            Time::M => write!(f, "m"),
            Time::S => write!(f, "s"),
            Time::Ms => write!(f, "ms"),
            Time::Micros => write!(f, "micros"),
            Time::Nanos => write!(f, "nanos"),
        }
    }
}
#[doc = "The type to sample (default: cpu)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum Type {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "gpu")]
    Gpu,
    #[serde(rename = "mem")]
    Mem,
}
impl ::std::fmt::Display for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Type::Cpu => write!(f, "cpu"),
            Type::Wait => write!(f, "wait"),
            Type::Block => write!(f, "block"),
            Type::Gpu => write!(f, "gpu"),
            Type::Mem => write!(f, "mem"),
        }
    }
}
#[doc = "Specific version type"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum VersionType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
}
impl ::std::fmt::Display for VersionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VersionType::Internal => write!(f, "internal"),
            VersionType::External => write!(f, "external"),
            VersionType::ExternalGte => write!(f, "external_gte"),
        }
    }
}
#[doc = "Specifies the allocation status to wait for before returning."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum WaitFor {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "fully_allocated")]
    FullyAllocated,
}
impl ::std::fmt::Display for WaitFor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            WaitFor::Started => write!(f, "started"),
            WaitFor::Starting => write!(f, "starting"),
            WaitFor::FullyAllocated => write!(f, "fully_allocated"),
        }
    }
}
#[doc = "Wait until all currently queued events with the given priority are processed"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
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
impl ::std::fmt::Display for WaitForEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            WaitForEvents::Immediate => write!(f, "immediate"),
            WaitForEvents::Urgent => write!(f, "urgent"),
            WaitForEvents::High => write!(f, "high"),
            WaitForEvents::Normal => write!(f, "normal"),
            WaitForEvents::Low => write!(f, "low"),
            WaitForEvents::Languid => write!(f, "languid"),
        }
    }
}
#[doc = "Wait until cluster is in a specific state"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
pub enum WaitForStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unavailable")]
    Unavailable,
}
impl ::std::fmt::Display for WaitForStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            WaitForStatus::Green => write!(f, "green"),
            WaitForStatus::Yellow => write!(f, "yellow"),
            WaitForStatus::Red => write!(f, "red"),
            WaitForStatus::Unknown => write!(f, "unknown"),
            WaitForStatus::Unavailable => write!(f, "unavailable"),
        }
    }
}
// GENERATED-END

/// Control how the total number of hits should be tracked.
///
/// When set to `Track` with a value `true`, the response will always track the number of hits that
/// match the query accurately.
///
/// When set to `Count` with an integer value `n`, the response accurately tracks the total
/// hit count that match the query up to `n` documents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackTotalHits {
    /// Whether to accurately track the number of hits that match the query accurately
    Track(bool),
    /// Accurately track the number of hits up to the specified value
    Count(i64),
}

impl From<bool> for TrackTotalHits {
    fn from(b: bool) -> Self {
        TrackTotalHits::Track(b)
    }
}

impl From<i64> for TrackTotalHits {
    fn from(i: i64) -> Self {
        TrackTotalHits::Count(i)
    }
}

/// Control how the `_source` field is returned with every hit.
///
/// By default operations return the contents of the `_source` field
/// unless you have used the `stored_fields` parameter or if the `_source` field is disabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceFilter {
    /// Whether `_source` retrieval should be enabled (`true`) or disabled (`false`)
    Enable(bool),

    /// A wildcard pattern to control what parts of `_source` should be returned
    Include(String),

    /// A collection of wildcard patterns to control what parts of `_source` should be returned
    Includes(Vec<String>),

    /// A collection of wildcard patterns to control what parts of `_source` should
    /// and should not be returned
    IncludesExcludes {
        includes: Vec<String>,
        excludes: Vec<String>,
    },
}

impl From<bool> for SourceFilter {
    fn from(b: bool) -> Self {
        SourceFilter::Enable(b)
    }
}

impl From<String> for SourceFilter {
    fn from(include: String) -> Self {
        SourceFilter::Include(include)
    }
}

impl<'a> From<&'a str> for SourceFilter {
    fn from(include: &'a str) -> Self {
        SourceFilter::Include(include.to_owned())
    }
}

impl From<Vec<String>> for SourceFilter {
    fn from(includes: Vec<String>) -> Self {
        SourceFilter::Includes(includes)
    }
}

impl<'a> From<Vec<&'a str>> for SourceFilter {
    fn from(includes: Vec<&'a str>) -> Self {
        SourceFilter::Includes(includes.iter().map(|s| (*s).to_string()).collect())
    }
}

impl From<(Vec<String>, Vec<String>)> for SourceFilter {
    fn from(includes_excludes: (Vec<String>, Vec<String>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes.0,
            excludes: includes_excludes.1,
        }
    }
}

impl<'a> From<(Vec<&'a str>, Vec<&'a str>)> for SourceFilter {
    fn from(includes_excludes: (Vec<&'a str>, Vec<&'a str>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes
                .0
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            excludes: includes_excludes
                .1
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        }
    }
}

/// Control the number of slices a task should be divided into. Defaults to `Slices::Count(1)`,
/// meaning the task is not sliced.
///
/// When set to `Auto`, a task is automatically divided into a reasonable number of slices
///
/// When set to `Count` with an integer value `n`, divides the task into that number of slices
#[derive(Debug, Clone, PartialEq)]
pub enum Slices {
    /// Automatically divide the task into a reasonable number of slices
    Auto,
    /// Number of slices to divide a task into
    Count(i32),
}

impl Serialize for Slices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Slices::Auto => serializer.serialize_str("auto"),
            Slices::Count(i) => serializer.serialize_i32(i),
        }
    }
}

impl<'de> Deserialize<'de> for Slices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlicesVisitor;

        impl<'de> Visitor<'de> for SlicesVisitor {
            type Value = Slices;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "expected integer or string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i32::MAX as i64 {
                    Ok(Slices::Count(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i32::MAX as u64 {
                    Ok(Slices::Count(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "auto" => Ok(Slices::Auto),
                    n => match n.parse::<i32>() {
                        Ok(i) => Ok(Slices::Count(i)),
                        Err(_) => Err(E::custom(format!(
                            "expected 'auto' or i32 but received: {}",
                            n
                        ))),
                    },
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(SlicesVisitor)
    }
}

impl Default for Slices {
    fn default() -> Self {
        Slices::Count(1)
    }
}

impl From<i32> for Slices {
    fn from(i: i32) -> Self {
        Slices::Count(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Slices;

    #[test]
    fn serialize_slices_auto() {
        let json = serde_json::to_string(&Slices::Auto).unwrap();
        assert_eq!("\"auto\"", &json);
        let slices: Slices = serde_json::from_str(&json).unwrap();
        assert_eq!(Slices::Auto, slices);
    }

    #[test]
    fn serialize_slices_count() {
        let json = serde_json::to_string(&Slices::Count(100)).unwrap();
        assert_eq!("100", &json);
        let slices: Slices = serde_json::from_str(&json).unwrap();
        assert_eq!(Slices::Count(100), slices);
    }
}
