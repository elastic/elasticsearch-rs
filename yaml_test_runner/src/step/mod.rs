use api_generator::generator::Api;
use std::fmt::Write;
use yaml_rust::Yaml;

mod r#do;
mod length;
mod r#match;
mod set;
mod skip;
pub use length::*;
pub use r#do::*;
pub use r#match::*;
pub use set::*;
pub use skip::*;

pub fn parse_steps(api: &Api, steps: &[Yaml]) -> Result<Vec<Step>, failure::Error> {
    let mut parsed_steps: Vec<Step> = Vec::new();
    for step in steps {
        let hash = step
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("Expected hash but found {:?}", step)))?;

        let (key, value) = {
            let (k, yaml) = hash.iter().next().unwrap();
            let key = k.as_str().ok_or_else(|| {
                failure::err_msg(format!("Expected string key but found {:?}", k))
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
            "transform_and_set" => {}
            "match" => {
                let m = Match::try_parse(value)?;
                parsed_steps.push(m.into());
            }
            "contains" => {}
            "is_true" => {}
            "is_false" => {}
            "length" => {
                let l = Length::try_parse(value)?;
                parsed_steps.push(l.into())
            }
            "eq" => {}
            "gte" => {}
            "lte" => {}
            "gt" => {}
            "lt" => {}
            op => return Err(failure::err_msg(format!("unknown step operation: {}", op))),
        }
    }

    Ok(parsed_steps)
}

pub trait BodyExpr {
    fn is_body_expr(&self, key: &str) -> bool {
        key == "$body"
    }

    /// Builds an indexer expression from the match key e.g.
    /// match key `2.airline` is converted to `[2]["airline"]`
    fn body_expr(&self, key: &str) -> String {
        if self.is_body_expr(key) {
            key.into()
        } else {
            let mut values = Vec::new();
            let mut value = String::new();
            let mut chars = key.chars();
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
            if self.is_body_expr(values[0].as_ref()) {
                values.remove(0);
            }

            let mut expr = String::new();
            for s in values {
                if s.chars().all(char::is_numeric) {
                    write!(expr, "[{}]", s).unwrap();
                } else if s.starts_with('$') {
                    // handle set values
                    let t = s
                        .trim_start_matches('$')
                        .trim_start_matches('{')
                        .trim_end_matches('}');
                    write!(expr, "[{}.as_str().unwrap()]", t).unwrap();
                } else {
                    write!(expr, "[\"{}\"]", s).unwrap();
                }
            }
            expr
        }
    }
}

pub enum Step {
    Skip(Skip),
    Set(Set),
    Do(Do),
    Match(Match),
    Length(Length),
}

impl Step {
    pub fn r#do(&self) -> Option<&Do> {
        match self {
            Step::Do(d) => Some(d),
            _ => None,
        }
    }
}

/// Checks whether there are any Errs in the collection, and accumulates them into one
/// error message if there are.
pub fn ok_or_accumulate<T>(
    results: &[Result<T, failure::Error>],
    indent: usize,
) -> Result<(), failure::Error> {
    let errs = results
        .iter()
        .filter_map(|r| r.as_ref().err())
        .collect::<Vec<_>>();
    if errs.is_empty() {
        Ok(())
    } else {
        let msg = errs
            .iter()
            .map(|e| format!("{}{}", "  ".to_string().repeat(indent), e.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        Err(failure::err_msg(msg))
    }
}

// trim the enclosing forward slashes and
// 1. replace escaped forward slashes (not needed after trimming forward slashes)
// 2. replace escaped colons (not supported by regex crate)
pub fn clean_regex<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .trim()
        .trim_matches('/')
        .replace("\\/", "/")
        .replace("\\:", ":")
        .replace("\\#", "#")
}
