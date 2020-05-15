#![allow(unused_macros)]
#![macro_use]

#[macro_export]
macro_rules! assert_response_success {
    ($response:ident) => {{
        assert!(
            $response.status_code().is_success(),
            "expected response to be successful but was {}",
            $response.status_code().as_u16()
        );
    }};
}

#[macro_export]
macro_rules! assert_response_success_or {
    ($response:ident, $status:expr) => {{
        assert!(
            $response.status_code().is_success() || $response.status_code().as_u16() == $status,
            "expected response to be successful or {} but was {}",
            $status,
            $response.status_code().as_u16()
        );
    }};
}

#[macro_export]
macro_rules! assert_match {
    ($expected:expr, $($actual:tt)+) => {{
        assert_eq!($expected, json!($($actual)+),
            "expected value {} to match {:?} but was {:?}", stringify!($expected), json!($($actual)+), $expected
        );
    }};
}

/// handle the case where the YAML test asserts a match against an integer value
/// but a floating point value is returned from Elasticsearch
#[macro_export]
macro_rules! assert_numeric_match {
    ($expected:expr, $actual:expr) => {{
        if $expected.is_i64() {
            assert_match!($expected, $actual);
        } else {
            assert_match!($expected, $actual as f64);
        }
    }};
}

#[macro_export]
macro_rules! assert_null {
    ($expected:expr) => {{
        assert!($expected.is_null(), "expected value {} to be null but was {:?}", stringify!($expected), $expected);
    }};
}

/// Asserts that the first string value matches the second string regular expression
#[macro_export]
macro_rules! assert_regex_match {
    ($expected:expr, $regex:expr) => {{
        let regex = regex::RegexBuilder::new($regex)
            .ignore_whitespace(true)
            .build()?;
        assert!(
            regex.is_match($expected),
            "expected value {} to match regex\n\n{}\n\nbut was\n\n{}",
            stringify!($expected),
            $regex,
            $expected
        );
    }};
}

/// Asserts that the length of a [serde_json::Value] type matches the expected length.
/// A length is calculated from the value based on the variant e.g.
/// - string length
/// - array length
/// - number of keys in object
/// - numeric value
#[macro_export]
macro_rules! assert_length {
    ($expr:expr, $len:expr) => {{
        let len = match $expr {
            Value::Number(n) => n.as_i64().unwrap() as usize,
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            v => panic!("Cannot get length from {:?}", v)
        };

        assert_eq!(
            $len,
            len,
            "expected value {} to have length {} but was {}",
            stringify!($expr),
            $len,
            len
        );
    }};
}