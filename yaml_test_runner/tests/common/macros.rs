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
/// Asserts that a [Response] has a status code >=200 and <300

#[macro_export]
macro_rules! assert_response_success {
    ($response:ident) => {{
        let status_code = $response.status_code();
        if !status_code.is_success() {
            let text = $response
                .text()
                .await
                .unwrap_or("[no response]".to_string());
            panic!(
                "expected response to be successful but was {}. Response: {}",
                status_code, text
            );
        }
    }};
}

/// Asserts that a [Response] has a status code >=200 and <300 or matches the passed status
#[macro_export]
macro_rules! assert_response_success_or {
    ($response:ident, $status:expr) => {{
        let status_code = $response.status_code();
        if !status_code.is_success() && status_code.as_u16() != $status {
            let text = $response
                .text()
                .await
                .unwrap_or("[no response]".to_string());
            panic!(
                "expected response to be successful or {} but was {}",
                $status,
                status_code.as_u16()
            );
        }
    }};
}

/// Asserts that a [Response] has a status that matches the passed status
#[macro_export]
macro_rules! assert_response_status_code {
    ($response:ident, $status:expr) => {{
        let status_code = $response.status_code();
        if status_code.as_u16() != $status {
            let text = $response
                .text()
                .await
                .unwrap_or("[no response]".to_string());
            panic!(
                "expected response to be {} but was {}",
                $status,
                status_code.as_u16()
            );
        }
    }};
}

#[macro_export]
macro_rules! assert_status_code {
    ($status_code:expr, $expected:expr) => {{
        assert_eq!(
            $expected,
            $status_code.as_u16(),
            "expected status code to be {} but was {}",
            $expected,
            $status_code.as_u16()
        );
    }};
}

#[macro_export]
macro_rules! assert_request_status_code {
    ($status_code:expr) => {{
        let status_code = $status_code.as_u16();
        assert!(
            status_code >= 400 && status_code < 600,
            "expected status code in range 400-599 but was {}",
            status_code
        );
    }};
}

/// Asserts that the passed [serde_json::Value] matches the second argument.
/// The second argument is converted to a [serde_json::Value] using the `json!` macro
#[macro_export]
macro_rules! assert_match {
    ($expected:expr, $actual:expr) => {{
        assert_eq!(
            $expected,
            $actual,
            "expected value {} to match {:?} but was {:?}",
            stringify!($expected),
            $actual,
            $expected
        );
    }};
}

/// Asserts that the passed [serde_json::Value] matches the expected numeric value.
/// This handles the case where a YAML test asserts a match against an integer value
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

/// Asserts that a [serde_json::Value] is null.
#[macro_export]
macro_rules! assert_null {
    ($expected:expr) => {{
        assert!(
            $expected.is_null(),
            "expected value {} to be null but was {:?}",
            stringify!($expected),
            $expected
        );
    }};
}

/// Asserts that the first string value matches the second string regular expression. An optional
/// third bool argument ignores pattern whitespace.
#[macro_export]
macro_rules! assert_regex_match {
    ($expected:expr, $regex:expr) => {{
        let regex = regex::RegexBuilder::new($regex).build()?;
        assert!(
            regex.is_match($expected),
            "expected value {} to match regex\n\n{}\n\nbut was\n\n{}",
            stringify!($expected),
            $regex,
            $expected
        );
    }};
    ($expected:expr, $regex:expr, $ignore_whitespace:expr) => {{
        let regex = regex::RegexBuilder::new($regex)
            .ignore_whitespace($ignore_whitespace)
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

/// Asserts that the length of a [serde_json::Value] matches the expected length.
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
            v => panic!("Cannot get length from {:?}", v),
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

/// Asserts that the expression is "false" i.e. `0`, `false`, `undefined`, `null` or `""`
#[macro_export]
macro_rules! assert_is_false {
    ($expr:expr) => {{
        let expr_string = stringify!($expr);
        match $expr {
            Value::Null => {}
            Value::Bool(b) => assert_eq!(
                *b, false,
                "expected value at {} to be false but was {}",
                expr_string, b
            ),
            Value::Number(n) => assert_eq!(
                n.as_f64().unwrap(),
                0.0,
                "expected value at {} to be false (0) but was {}",
                expr_string,
                n.as_f64().unwrap()
            ),
            Value::String(s) => assert!(
                s.is_empty(),
                "expected value at {} to be false (empty) but was {}",
                expr_string,
                &s
            ),
            v => assert!(
                false,
                "expected value at {} to be false but was {:?}",
                expr_string, &v
            ),
        }
    }};
}

/// Asserts that the expression is "true" i.e. not `0`, `false`, `undefined`, `null` or `""`
#[macro_export]
macro_rules! assert_is_true {
    ($expr:expr) => {{
        let expr_string = stringify!($expr);
        match $expr {
            Value::Null => assert!(
                false,
                "expected value at {} to be true (not null) but was null",
                expr_string
            ),
            Value::Bool(b) => assert!(
                *b,
                "expected value at {} to be true but was false",
                expr_string
            ),
            Value::Number(n) => assert_ne!(
                n.as_f64().unwrap(),
                0.0,
                "expected value at {} to be true (not 0) but was {}",
                expr_string,
                n.as_f64().unwrap()
            ),
            Value::String(s) => assert!(
                !s.is_empty(),
                "expected value at {} to be true (not empty) but was {}",
                expr_string,
                &s
            ),
            v => {}
        }
    }};
}

/// Asserts that the deprecation warnings contain a given value
#[macro_export]
macro_rules! assert_warnings_contain {
    ($warnings:expr, $expected:expr) => {{
        assert!(
            $warnings.iter().any(|w| w.contains($expected)),
            "expected warnings to contain '{}' but contained {:?}",
            $expected,
            &$warnings
        );
    }};
}

/// Asserts that the deprecation warnings are empty
#[macro_export]
macro_rules! assert_warnings_is_empty {
    ($warnings:expr) => {{
        assert!(
            $warnings.is_empty(),
            "expected warnings to be empty but found {:?}",
            &$warnings
        );
    }};
}

/// Asserts that the comparison is true
#[macro_export]
macro_rules! assert_comparison {
    ($expr:expr, $($comparison:tt)+) => {{
        match $expr {
            Value::Number(n) => {
                match n.as_i64() {
                    Some(i) => assert!(i $($comparison)+ as i64, "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+ as i64), i),
                    None => match n.as_f64() {
                        Some(f) => assert!(f $($comparison)+ as f64, "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+ as f64), f),
                        None => match n.as_u64() {
                            Some(u) => assert!(u $($comparison)+ as u64, "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+ as u64), u),
                            None => assert!(false, "Expected value {} to be numeric but was {:?}", stringify!($expr), &n)
                        }
                    }
                }
            }
            v => assert!(false, "Expected value {} to be numeric but was {:?}", stringify!($expr), &v),
        }
    }};
}

/// Asserts that the comparison is true when comparing against a "set" value
#[macro_export]
macro_rules! assert_comparison_from_set_value {
    ($expr:expr, $($comparison:tt)+) => {{
        match $expr {
            Value::Number(n) => {
                match n.as_i64() {
                    Some(i) => assert!(i $($comparison)+.as_i64().unwrap(), "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+.as_i64().unwrap()), i),
                    None => match n.as_f64() {
                        Some(f) => assert!(f $($comparison)+.as_f64().unwrap(), "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+.as_f64().unwrap()), f),
                        None => match n.as_u64() {
                            Some(u) => assert!(u $($comparison)+.as_u64().unwrap(), "Expected value {} to be {} but was {}", stringify!($expr), stringify!($($comparison)+.as_u64().unwrap()), u),
                            None => assert!(false, "Expected value {} to be numeric but was {:?}", stringify!($expr), &n)
                        }
                    }
                }
            }
            v => assert!(false, "Expected value {} to be numeric but was {:?}", stringify!($expr), &v),
        }
    }};
}

/// Asserts that the passed [serde_json::Value::Array] contains the second argument.
#[macro_export]
macro_rules! assert_contains {
    ($expr:expr, $value:expr) => {{
        if !$expr.is_array() {
            assert!(
                false,
                "expected {} to be an array but was {:?}",
                stringify!($expr),
                &$expr
            );
        }

        let arr = $expr.as_array().unwrap();

        // when dealing with a serde_json::Value::Object, the $value may only be a partial object
        // such that equality can't be used. In this case, we need to assert that there is one
        // object in the array that has all the keys and values of $value
        if $value.is_object() {
            let vv = $value.clone();
            let o = vv.as_object().unwrap();
            assert!(
                arr.iter()
                    .filter_map(serde_json::Value::as_object)
                    .any(|ao| o
                        .iter()
                        .all(|(key, value)| ao.get(key).map_or(false, |v| *value == *v))),
                "expected value {} to contain {:?} but contained {:?}",
                stringify!($expr),
                &vv,
                &arr
            );
        } else {
            assert!(
                arr.contains(&$value),
                "expected value {} to contain {:?} but contained {:?}",
                stringify!($expr),
                &$value,
                &arr
            );
        }
    }};
}
