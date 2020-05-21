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
use super::Step;
use regex::Regex;
use yaml_rust::Yaml;

pub struct Skip {
    version_requirements: Option<semver::VersionReq>,
    version: Option<String>,
    reason: Option<String>,
    features: Option<Vec<String>>,
}

impl From<Skip> for Step {
    fn from(skip: Skip) -> Self {
        Step::Skip(skip)
    }
}

impl Skip {
    /// Gets the version. Returns empty if no version
    pub fn version(&self) -> String {
        self.version.clone().unwrap_or_else(|| "".into())
    }

    /// Gets the reason. Returns empty string if no reason
    pub fn reason(&self) -> String {
        self.reason.clone().unwrap_or_else(|| "".into())
    }

    /// Gets the features. Returns empty slice if no features
    pub fn features(&self) -> &[String] {
        match &self.features {
            Some(v) => v,
            None => &[],
        }
    }

    /// Converts the version range specified in the yaml test into a [semver::VersionReq]
    fn parse_version_requirements(version: &Option<String>) -> Option<semver::VersionReq> {
        if let Some(v) = version {
            if v.to_lowercase() == "all" {
                Some(semver::VersionReq::any())
            } else {
                lazy_static! {
                    static ref VERSION_REGEX: Regex =
                        Regex::new(r"^([\w\.]+)?\s*?\-\s*?([\w\.]+)?$").unwrap();
                }
                if let Some(c) = VERSION_REGEX.captures(v) {
                    match (c.get(1), c.get(2)) {
                        (Some(start), Some(end)) => Some(
                            semver::VersionReq::parse(
                                format!(">={},<={}", start.as_str(), end.as_str()).as_ref(),
                            )
                            .unwrap(),
                        ),
                        (Some(start), None) => Some(
                            semver::VersionReq::parse(format!(">={}", start.as_str()).as_ref())
                                .unwrap(),
                        ),
                        (None, Some(end)) => Some(
                            semver::VersionReq::parse(format!("<={}", end.as_str()).as_ref())
                                .unwrap(),
                        ),
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn try_parse(yaml: &Yaml) -> Result<Skip, failure::Error> {
        let version = yaml["version"]
            .as_str()
            .map_or_else(|| None, |y| Some(y.to_string()));
        let reason = yaml["reason"]
            .as_str()
            .map_or_else(|| None, |y| Some(y.to_string()));
        let features = match &yaml["features"] {
            Yaml::String(s) => Some(vec![s.to_string()]),
            Yaml::Array(a) => Some(
                a.iter()
                    .map(|y| y.as_str().map(|s| s.to_string()).unwrap())
                    .collect(),
            ),
            _ => None,
        };

        let version_requirements = Self::parse_version_requirements(&version);

        Ok(Skip {
            version,
            version_requirements,
            reason,
            features,
        })
    }

    /// Determines if this instance matches the version
    pub fn skip_version(&self, version: &semver::Version) -> bool {
        match &self.version_requirements {
            Some(r) => r.matches(version),
            None => false,
        }
    }

    /// Determines if this instance matches the version
    pub fn skip_features(&self, features: &[String]) -> bool {
        match &self.features {
            Some(test_features) => test_features.iter().any(|f| features.contains(f)),
            None => false,
        }
    }
}
