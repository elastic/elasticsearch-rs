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
//! These tests require a cluster configured with Security. One can be spun up using the
//! .ci/run-elasticsearch.sh script as follows:
//!
//! export TEST_SUITE=xpack
//! export STACK_VERSION=<a version that aligns with the client e.g. 7.8.0, 8.0.0-SNAPSHOT, etc>
//!
//! DETACH=true .ci/run-elasticsearch.sh
#![cfg(any(feature = "native-tls", feature = "rustls-tls"))]

extern crate os_type;

pub mod common;
use common::*;

use elasticsearch::cert::{Certificate, CertificateValidation};
use os_type::OSType;

static CA_CERT: &[u8] = include_bytes!("../../.ci/certs/ca.crt");
static CA_CHAIN_CERT: &[u8] = include_bytes!("../../.ci/certs/ca-chain.crt");
static TESTNODE_CERT: &[u8] = include_bytes!("../../.ci/certs/testnode.crt");
static TESTNODE_NO_SAN_CERT: &[u8] = include_bytes!("../../.ci/certs/testnode_no_san.crt");

fn expected_error_message() -> String {
    if cfg!(windows) {
        "terminated in a root certificate which is not trusted by the trust provider".to_string()
    } else {
        let os = os_type::current_platform();
        match os.os_type {
            OSType::OSX => "The certificate was not trusted".to_string(),
            _ => "unable to get local issuer certificate".to_string(),
        }
    }
}

/// Default certificate validation with a self signed certificate
#[tokio::test]
#[cfg(feature = "native-tls")]
async fn default_certificate_validation() -> Result<(), failure::Error> {
    let builder = client::create_default_builder().cert_validation(CertificateValidation::Default);
    let client = client::create(builder);
    let result = client.ping().send().await;

    match result {
        Ok(response) => Err(failure::err_msg(format!(
            "Expected error but response was {}",
            response.status_code()
        ))),
        Err(e) => {
            let expected = expected_error_message();
            let actual = e.to_string();
            assert!(
                actual.contains(&expected),
                "Expected error message to contain '{}' but was '{}'",
                expected,
                actual
            );
            Ok(())
        }
    }
}

/// Default certificate validation with a self signed certificate and rustls-tls
#[tokio::test]
#[cfg(all(feature = "rustls-tls", not(feature = "native-tls")))]
async fn default_certificate_validation_rustls_tls() -> Result<(), failure::Error> {
    let builder = client::create_default_builder().cert_validation(CertificateValidation::Default);
    let client = client::create(builder);
    let result = client.ping().send().await;

    match result {
        Ok(response) => Err(failure::err_msg(format!(
            "Expected error but response was {}",
            response.status_code()
        ))),
        Err(e) => {
            let expected = "invalid certificate: UnknownIssuer";
            let actual = e.to_string();
            assert!(
                actual.contains(&expected),
                "Expected error message to contain '{}' but was '{}'",
                expected,
                actual
            );
            Ok(())
        }
    }
}

/// Allows any certificate through
#[tokio::test]
async fn none_certificate_validation() -> Result<(), failure::Error> {
    let builder = client::create_default_builder().cert_validation(CertificateValidation::None);
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server contains the one given to the client
/// within the authority chain, and hostname matches
#[tokio::test]
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
async fn full_certificate_ca_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(CA_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Try to load a certificate chain.
#[tokio::test]
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
async fn full_certificate_ca_chain_validation() -> Result<(), failure::Error> {
    let mut cert = Certificate::from_pem(CA_CHAIN_CERT)?;
    cert.append(Certificate::from_pem(CA_CERT)?);
    assert_eq!(cert.len(), 3, "expected three certificates in CA chain");
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client and hostname matches
#[tokio::test]
#[cfg(all(windows, feature = "native-tls"))]
async fn full_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client and hostname matches, using rustls-tls
#[tokio::test]
#[cfg(feature = "rustls-tls")]
async fn full_certificate_validation_rustls_tls() -> Result<(), failure::Error> {
    let mut chain: Vec<u8> = Vec::with_capacity(TESTNODE_CERT.len() + CA_CERT.len());
    chain.extend(CA_CERT);
    chain.extend(TESTNODE_CERT);

    let cert = Certificate::from_pem(chain.as_slice())?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(all(unix, any(feature = "native-tls", feature = "rustls-tls")))]
async fn full_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let result = client.ping().send().await;
    let os_type = os_type::current_platform();
    match os_type.os_type {
        OSType::OSX => match result {
            Ok(_) => Ok(()),
            Err(e) => Err(failure::err_msg(e.to_string())),
        },
        _ => match result {
            Ok(response) => Err(failure::err_msg(format!(
                "Expected error but response was {}",
                response.status_code()
            ))),
            Err(e) => {
                let expected = expected_error_message();
                let actual = e.to_string();
                assert!(
                    actual.contains(&expected),
                    "Expected error message to contain '{}' but was '{}'",
                    expected,
                    actual
                );
                Ok(())
            }
        },
    }
}

/// Certificate provided by the server is the one given to the client
#[tokio::test]
#[cfg(all(windows, feature = "native-tls"))]
async fn certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Certificate(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(all(unix, feature = "native-tls"))]
async fn certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Certificate(cert));
    let client = client::create(builder);
    let result = client.ping().send().await;
    let os_type = os_type::current_platform();
    match os_type.os_type {
        OSType::OSX => match result {
            Ok(_) => Ok(()),
            Err(e) => Err(failure::err_msg(e.to_string())),
        },
        _ => match result {
            Ok(response) => Err(failure::err_msg(format!(
                "Expected error but response was {}",
                response.status_code()
            ))),
            Err(e) => {
                let expected = expected_error_message();
                let actual = e.to_string();
                assert!(
                    actual.contains(&expected),
                    "Expected error message to contain '{}' but was '{}'",
                    expected,
                    actual
                );
                Ok(())
            }
        },
    }
}

/// Certificate provided by the server contains the one given to the client
/// within the authority chain
#[tokio::test]
#[cfg(feature = "native-tls")]
async fn certificate_certificate_ca_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(CA_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Certificate(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server does not match the one given to the client
#[tokio::test]
#[cfg(feature = "native-tls")]
async fn fail_certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_NO_SAN_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Certificate(cert));

    let client = client::create(builder);
    let result = client.ping().send().await;

    match result {
        Ok(response) => Err(failure::err_msg(format!(
            "Expected error but response was {}",
            response.status_code()
        ))),
        Err(e) => {
            let expected = expected_error_message();
            let actual = e.to_string();
            assert!(
                actual.contains(&expected),
                "Expected error message to contain '{}' but was '{}'",
                expected,
                actual
            );
            Ok(())
        }
    }
}
