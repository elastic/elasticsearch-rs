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
pub mod common;
use common::*;

use elasticsearch::auth::Credentials;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, write::EncoderWriter};
use std::io::Write;
use std::ops::Deref;

#[tokio::test]
async fn basic_auth_header() -> Result<(), failure::Error> {
    eprintln!("Got tracker: {:?}", common::TRACKER.deref());
    let server = server::http(move |req| async move {
        let mut header_value = b"Basic ".to_vec();
        {
            let mut encoder = EncoderWriter::new(&mut header_value, &BASE64_STANDARD);
            write!(encoder, "username:password").unwrap();
        }

        assert_eq!(
            req.headers()["authorization"],
            String::from_utf8(header_value).unwrap()
        );
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .auth(Credentials::Basic("username".into(), "password".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn api_key_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        let mut header_value = b"ApiKey ".to_vec();
        {
            let mut encoder = EncoderWriter::new(&mut header_value, &BASE64_STANDARD);
            write!(encoder, "id:api_key").unwrap();
        }

        assert_eq!(
            req.headers()["authorization"],
            String::from_utf8(header_value).unwrap()
        );
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .auth(Credentials::ApiKey("id".into(), "api_key".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn encoded_api_key_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        let mut header_value = b"ApiKey ".to_vec();
        {
            let mut encoder = EncoderWriter::new(&mut header_value, &BASE64_STANDARD);
            write!(encoder, "id:api_key").unwrap();
        }

        assert_eq!(
            req.headers()["authorization"],
            String::from_utf8(header_value).unwrap()
        );
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        // result of `echo -n "id:api_key" | base64`
        .auth(Credentials::EncodedApiKey("aWQ6YXBpX2tleQ==".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn bearer_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| async move {
        assert_eq!(req.headers()["authorization"], "Bearer access_token");
        http::Response::default()
    });

    let builder = client::create_builder(format!("http://{}", server.addr()).as_ref())
        .auth(Credentials::Bearer("access_token".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

// TODO: test PKI authentication. Could configure a HttpsConnector, maybe using https://github.com/sfackler/hyper-openssl?, or send to PKI configured Elasticsearch.
//#[tokio::test]
//async fn client_certificate() -> Result<(), failure::Error> {
//    let server = server::http(move |req| {
//        async move {
//            http::Response::default()
//        }
//    });
//
//    let mut buf = Vec::new();
//    File::open("common/client.p12")?
//        .read_to_end(&mut buf)?;
//
//    let builder = client::create_conn_builder(format!("https://{}", server.addr()).as_ref())
//        .auth(Credentials::Certificate(buf, "".into()));
//
//    let client = client::create(builder);
//    let _response = client
//        .ping()
//        .send()
//        .await?;
//
//    Ok(())
//}
