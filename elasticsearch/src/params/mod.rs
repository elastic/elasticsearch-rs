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

pub use super::generated::params::*;
use core::fmt;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

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
