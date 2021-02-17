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
#[doc = "The unit in which to display byte values"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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
#[doc = "What to do when the delete by query hits version conflicts?"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Conflicts {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "proceed")]
    Proceed,
}
#[doc = "The default operator for query string query (AND or OR)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum DefaultOperator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}
#[doc = "Whether to expand wildcard expression to concrete indices that are open, closed or both."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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
#[doc = "Optional parameter to specify the high level file format"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Format {
    #[serde(rename = "ndjson")]
    Ndjson,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "delimited")]
    Delimited,
    #[serde(rename = "semi_structured_text")]
    SemiStructuredText,
}
#[doc = "Group tasks by nodes or parent/child relationships"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum GroupBy {
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "parents")]
    Parents,
    #[serde(rename = "none")]
    None,
}
#[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Health {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
#[doc = "Specify the level of detail for returned information"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Level {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "indices")]
    Indices,
    #[serde(rename = "shards")]
    Shards,
}
#[doc = "Explicit operation type. Defaults to `index` for requests with an explicit document ID, and to `create`for requests without an explicit document ID"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum OpType {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "create")]
    Create,
}
#[doc = "If `true` (the default) then refresh the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` then do nothing with refreshes."]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Refresh {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
    #[serde(rename = "wait_for")]
    WaitFor,
}
#[doc = "Search operation type"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum SearchType {
    #[serde(rename = "query_then_fetch")]
    QueryThenFetch,
    #[serde(rename = "dfs_query_then_fetch")]
    DfsQueryThenFetch,
}
#[doc = "The multiplier in which to display values"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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
#[doc = "Specify suggest mode"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum SuggestMode {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "always")]
    Always,
}
#[doc = "The unit in which to display time values"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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
#[doc = "The type to sample (default: cpu)"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Type {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "block")]
    Block,
}
#[doc = "Specific version type"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum VersionType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
    #[serde(rename = "force")]
    Force,
}
#[doc = "Wait until all currently queued events with the given priority are processed"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
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
#[doc = "Wait until cluster is in a specific state"]
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum WaitForStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
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
                if value <= i32::max_value() as i64 {
                    Ok(Slices::Count(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i32::max_value() as u64 {
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
