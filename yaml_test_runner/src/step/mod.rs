use api_generator::generator::Api;
use yaml_rust::Yaml;

mod r#do;
mod r#match;
mod skip;
pub use r#do::*;
pub use r#match::*;
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
            "set" => {}
            "transform_and_set" => {}
            "match" => {
                let m = Match::try_parse(value)?;
                parsed_steps.push(m.into());
            }
            "contains" => {}
            "is_true" => {}
            "is_false" => {}
            "length" => {}
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

pub enum Step {
    Skip(Skip),
    Do(Do),
    Match(Match),
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
            .map(|e| format!("{}{}", "\t".to_string().repeat(indent), e.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        Err(failure::err_msg(msg))
    }
}
