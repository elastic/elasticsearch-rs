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
use crate::regex::*;
use api_generator::generator::Api;
use std::fmt::Write;
use yaml_rust::{Yaml, YamlEmitter};

mod comparison;
mod contains;
mod r#do;
mod is_false;
mod is_true;
mod length;
mod r#match;
mod set;
mod skip;
mod transform_and_set;
pub use comparison::{Comparison, OPERATORS};
pub use contains::*;
pub use is_false::*;
pub use is_true::*;
pub use length::*;
pub use r#do::*;
pub use r#match::*;
pub use set::*;
pub use skip::*;
pub use transform_and_set::*;

pub fn parse_steps(api: &Api, steps: &[Yaml]) -> Result<Vec<Step>, failure::Error> {
    let mut parsed_steps: Vec<Step> = Vec::new();
    for step in steps {
        let hash = step
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", step)))?;

        let (key, value) = {
            let (k, yaml) = hash.iter().next().unwrap();
            let key = k.as_str().ok_or_else(|| {
                failure::err_msg(format!("expected string key but found {:?}", k))
            })?;

            (key, yaml)
        };

        match key {
            "skip" => {
                let skip = Skip::try_parse(value)?;
                parsed_steps.push(skip.into());
            }
            "do" => {
                let d = Do::try_parse(api, value)?;
                parsed_steps.push(d.into())
            }
            "set" => {
                let s = Set::try_parse(value)?;
                parsed_steps.push(s.into());
            }
            "transform_and_set" => {
                let t = TransformAndSet::try_parse(value)?;
                parsed_steps.push(t.into());
            }
            "match" => {
                let m = Match::try_parse(value)?;
                parsed_steps.push(m.into());
            }
            "contains" => {
                let c = Contains::try_parse(value)?;
                parsed_steps.push(c.into());
            }
            "is_true" => {
                let e = IsTrue::try_parse(value)?;
                parsed_steps.push(e.into())
            }
            "is_false" => {
                let e = IsFalse::try_parse(value)?;
                parsed_steps.push(e.into())
            }
            "length" => {
                let l = Length::try_parse(value)?;
                parsed_steps.push(l.into())
            }
            op if OPERATORS.contains(&op) => {
                let comp = Comparison::try_parse(value, op)?;
                parsed_steps.push(comp.into())
            }
            op => return Err(failure::err_msg(format!("unknown step operation: {}", op))),
        }
    }

    Ok(parsed_steps)
}

/// An expression to apply to the response. Can be the whole body ($body or "") or an
/// indexer expression into a JSON response.
pub struct Expr {
    expr: String,
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        Expr::new(s)
    }
}

impl Expr {
    pub fn new<S: Into<String>>(expr: S) -> Self {
        Self { expr: expr.into() }
    }

    /// Whether the expression is "$body" or "", which are both used to express the whole body
    pub fn is_body(&self) -> bool {
        Self::is_string_body(&self.expr) || self.expr.is_empty()
    }

    fn is_string_body(s: &str) -> bool {
        s == "$body"
    }

    pub fn expression(&self) -> String {
        if self.is_body() {
            self.expr.clone()
        } else {
            let mut values = Vec::new();
            let mut value = String::new();
            let mut chars = self.expr.chars();
            while let Some(ch) = chars.next() {
                match ch {
                    '\\' => {
                        // consume the next character too
                        if let Some(next) = chars.next() {
                            value.push(next);
                        }
                    }
                    '.' => {
                        values.push(value);
                        value = String::new();
                    }
                    _ => {
                        value.push(ch);
                    }
                }
            }
            values.push(value);

            // some APIs specify the response body as the first part of the path
            // which should be removed.
            if Self::is_string_body(values[0].as_ref()) {
                values.remove(0);
            } else if values[0] == "" {
                // some tests start the json path with a dot, leading to an empty first element
                values.remove(0);
            }

            let mut expr = String::new();
            for s in values {
                if s.is_empty() {
                    write!(expr, "[\"\"]").unwrap();
                } else if s.chars().all(char::is_numeric) {
                    write!(expr, "[{}]", s).unwrap();
                } else if s.starts_with('$') {
                    // handle "set" values
                    let t = s
                        .trim_start_matches('$')
                        .trim_start_matches('{')
                        .trim_end_matches('}');
                    write!(expr, "[{}.as_str().unwrap()]", t).unwrap();
                } else if s.as_str() == "_arbitrary_key_" {
                    // handle _arbitrary_key_.
                    // wrap in Value::String to allow uniform unwrapping in subsequent steps
                    write!(
                        expr,
                        ".as_object().unwrap().iter().next().map(|(k, _)| json!(k)).unwrap()"
                    )
                    .unwrap();
                } else {
                    write!(expr, "[\"{}\"]", s).unwrap();
                }
            }
            expr
        }
    }
}

/// Steps defined in a yaml test
pub enum Step {
    Skip(Skip),
    Set(Set),
    Do(Do),
    Match(Match),
    Length(Length),
    IsTrue(IsTrue),
    IsFalse(IsFalse),
    Comparison(Comparison),
    Contains(Contains),
    TransformAndSet(TransformAndSet),
}

impl Step {
    /// Gets a Do step
    pub fn r#do(&self) -> Option<&Do> {
        match self {
            Step::Do(d) => Some(d),
            _ => None,
        }
    }
}

/// Checks whether there are any Errs in the collection, and accumulates them into one
/// error message if there are.
pub fn ok_or_accumulate<T>(results: &[Result<T, failure::Error>]) -> Result<(), failure::Error> {
    let errs = results
        .iter()
        .filter_map(|r| r.as_ref().err())
        .collect::<Vec<_>>();
    if errs.is_empty() {
        Ok(())
    } else {
        let mut msgs = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>();
        msgs.sort();
        msgs.dedup_by(|a, b| a == b);
        Err(failure::err_msg(msgs.join(", ")))
    }
}

pub fn json_string_from_yaml(yaml: &Yaml) -> String {
    let mut s = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut s);
        emitter.dump(yaml).unwrap();
    }

    let value: serde_json::Value = serde_yaml::from_str(&s).unwrap();

    let mut json = value.to_string();
    json = replace_set(json);
    json = replace_i64(json);
    json
}
