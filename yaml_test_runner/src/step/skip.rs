use super::Step;
use regex::Regex;
use yaml_rust::Yaml;

pub struct Skip {
    version_requirements: Option<semver::VersionReq>,
    pub version: Option<String>,
    pub reason: Option<String>,
    features: Option<Vec<String>>,
}

impl From<Skip> for Step {
    fn from(skip: Skip) -> Self {
        Step::Skip(skip)
    }
}

impl Skip {
    pub fn version(&self) -> String {
        self.version.clone().unwrap_or_else(|| "".into())
    }

    pub fn reason(&self) -> String {
        self.reason.clone().unwrap_or_else(|| "".into())
    }

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
    pub fn version_matches(&self, version: &semver::Version) -> bool {
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
