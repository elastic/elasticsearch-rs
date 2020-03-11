pub mod common;
use common::*;

use elasticsearch::cert::{Certificate, CertificateValidation};

// TODO: These tests require a cluster configured with Security. Figure out best way to surface this e.g. test category, naming convention, etc.

static CA_CERT: &[u8] = include_bytes!("../../.ci/certs/ca.crt");
static TESTNODE_SAN_CERT: &[u8] = include_bytes!("../../.ci/certs/testnode_san.crt");
static TESTNODE_CERT: &[u8] = include_bytes!("../../.ci/certs/testnode.crt");

fn expected_error_message() -> String {
    if cfg!(windows) {
        "terminated in a root certificate which is not trusted by the trust provider".to_string()
    } else {
        "unable to get local issuer certificate".to_string()
    }
}

/// Default certificate validation with a self signed certificate
#[tokio::test]
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
async fn full_certificate_ca_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(CA_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client and hostname matches
#[tokio::test]
#[cfg(windows)]
async fn full_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_SAN_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(unix)]
async fn full_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_SAN_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Full(cert));
    let client = client::create(builder);
    let result = client.ping().send().await;
    match result {
        Ok(response) => Err(failure::err_msg(format!(
            "Expected error but response was {}",
            response.status_code()
        ))),
        Err(e) => {
            let expected = "unable to get local issuer certificate";
            let actual = e.to_string();
            assert!(
                actual.contains(expected),
                "Expected error message to contain '{}' but was '{}'",
                expected,
                actual
            );
            Ok(())
        }
    }
}

/// Certificate provided by the server is the one given to the client
#[tokio::test]
#[cfg(windows)]
async fn certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_SAN_CERT)?;
    let builder =
        client::create_default_builder().cert_validation(CertificateValidation::Certificate(cert));
    let client = client::create(builder);
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(unix)]
async fn certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_SAN_CERT)?;
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
            let expected = "unable to get local issuer certificate";
            let actual = e.to_string();
            assert!(
                actual.contains(expected),
                "Expected error message to contain '{}' but was '{}'",
                expected,
                actual
            );
            Ok(())
        }
    }
}

/// Certificate provided by the server contains the one given to the client
/// within the authority chain
#[tokio::test]
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
async fn fail_certificate_certificate_validation() -> Result<(), failure::Error> {
    let cert = Certificate::from_pem(TESTNODE_CERT)?;
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
