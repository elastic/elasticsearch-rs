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
use base64::write::EncoderWriter as Base64Encoder;
use std::io::Write;

pub fn base_64_encode_credentials(user: &str, password: &str) -> String {
    let mut value = Vec::new();
    {
        let mut encoder = Base64Encoder::new(&mut value, base64::STANDARD);
        write!(encoder, "{}:", user).unwrap();
        write!(encoder, "{}", password).unwrap();
    };
    String::from_utf8(value).unwrap()
}
