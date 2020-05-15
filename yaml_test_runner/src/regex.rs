use lazy_static;
use regex::{Captures, Regex};

lazy_static! {
    // replace usages of "$.*" with the captured value
    pub static ref SET_REGEX: Regex =
        Regex::new(r#""\$(.*?)""#).unwrap();

    // replace usages of ${.*} with the captured value
    pub static ref SET_QUOTED_DELIMITED_REGEX: Regex =
        Regex::new(r#""\$\{(.*?)\}""#).unwrap();

    // replace usages of ${.*} with the captured value
    pub static ref SET_DELIMITED_REGEX: Regex =
        Regex::new(r#"\$\{(.*?)\}"#).unwrap();

    // include i64 suffix on whole numbers
    pub static ref INT_REGEX: Regex =
        regex::Regex::new(r"(:\s?)(\d+?)([,\s?|\s*?}])").unwrap();
}

/// Replaces a "set" step value with a variable
pub fn replace_set_quoted_delimited<S: AsRef<str>>(s: S) -> String {
    SET_QUOTED_DELIMITED_REGEX
        .replace_all(s.as_ref(), "$1")
        .into_owned()
}

/// Replaces a "set" step value with a variable
pub fn replace_set_delimited<S: AsRef<str>>(s: S) -> String {
    SET_DELIMITED_REGEX
        .replace_all(s.as_ref(), "$1")
        .into_owned()
}

/// Replaces a "set" step value with a variable
pub fn replace_set<S: AsRef<str>>(s: S) -> String {
    SET_REGEX.replace_all(s.as_ref(), "$1").into_owned()
}

/// Replaces all integers in a string to suffix with i64, to ensure that numbers
/// larger than i32 will be handled correctly when passed to json! macro
pub fn replace_i64<S: AsRef<str>>(s: S) -> String {
    INT_REGEX
        .replace_all(s.as_ref(), |c: &Captures| match &c[2].parse::<i64>() {
            Ok(i) if *i > i32::max_value() as i64 => format!("{}{}i64{}", &c[1], &c[2], &c[3]),
            _ => format!("{}", &c[0]),
        })
        .into_owned()
}
