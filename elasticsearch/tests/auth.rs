pub mod support;
use support::*;

use elasticsearch::Credentials;

use base64;
use base64::write::EncoderWriter as Base64Encoder;
use std::fs::File;
use std::io::{Read, Write};

#[tokio::test]
async fn basic_auth_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| {
        async move {
            let mut header_value = b"Basic ".to_vec();
            {
                let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
                write!(encoder, "username:password").unwrap();
            }

            assert_eq!(
                req.headers()["authorization"],
                String::from_utf8(header_value).unwrap()
            );
            http::Response::default()
        }
    });

    let builder = client::create_conn_builder(format!("http://{}", server.addr()).as_ref())
        .auth(Credentials::Basic("username".into(), "password".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn api_key_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| {
        async move {
            let mut header_value = b"ApiKey ".to_vec();
            {
                let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
                write!(encoder, "id:api_key").unwrap();
            }

            assert_eq!(
                req.headers()["authorization"],
                String::from_utf8(header_value).unwrap()
            );
            http::Response::default()
        }
    });

    let builder = client::create_conn_builder(format!("http://{}", server.addr()).as_ref())
        .auth(Credentials::ApiKey("id".into(), "api_key".into()));

    let client = client::create(builder);
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn bearer_header() -> Result<(), failure::Error> {
    let server = server::http(move |req| {
        async move {
            assert_eq!(req.headers()["authorization"], "Bearer access_token");
            http::Response::default()
        }
    });

    let builder = client::create_conn_builder(format!("http://{}", server.addr()).as_ref())
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
//    File::open("support/client.p12")?
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
