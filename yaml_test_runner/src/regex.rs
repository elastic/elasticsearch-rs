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
use ::regex::{Captures, Regex};
use lazy_static;

lazy_static! {
    // replace usages of "$.*" with the captured value
    pub static ref SET_REGEX: Regex =
        Regex::new(r#""\$(.*?)""#).unwrap();

    // replace usages of "${.*}" with the captured value
    pub static ref SET_QUOTED_DELIMITED_REGEX: Regex =
        Regex::new(r#""\$\{(.*?)\}""#).unwrap();

    // replace usages of ${.*} with the captured value
    pub static ref SET_DELIMITED_REGEX: Regex =
        Regex::new(r#"\$\{(.*?)\}"#).unwrap();

    // include i64 suffix on whole numbers larger than i32
    // will match on numbers with 10 or more digits, with the replace
    // call testing against i32::max_value
    pub static ref INT_REGEX: Regex =
        regex::Regex::new(r"([,:\[{]\s*)(\d{10,}?)(\s*[,}\]])").unwrap();
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
            _ => c[0].to_string(),
        })
        .into_owned()
}
