//! Certificate components
pub use reqwest::Certificate;

/// Validation applied to a SSL/TLS certificate, to establish a HTTPS connection.
///
/// # Examples
///
/// ## Default
///
/// The client is configured by default to validate that a certificate used to establish a
/// HTTPS connection is one that is signed by a trusted Certificate Authority (CA) and passes
/// hostname verification. [CertificateValidation::Default] is a provided variant only to
/// be able to change from another validation mode back to the default.
///
/// ## Full validation
///
/// With Elasticsearch running at https://example.com, configured to use a certificate generated
/// with your own Certificate Authority (CA), and where the certificate contains a CommonName (CN)
/// or Subject Alternative Name (SAN) that matches the hostname of Elasticsearch
///
/// ```rust,norun
/// # use elasticsearch::{
/// #     auth::Credentials,
/// #     cert::{Certificate,CertificateValidation},
/// #     Error, Elasticsearch,
/// #     http::transport::{TransportBuilder,SingleNodeConnectionPool},
/// # };
/// # use std::fs::File;
/// # use std::io::Read;
/// # use url::Url;
/// # async fn run() -> Result<(), Error> {
/// let url = Url::parse("https://example.com")?;
/// let conn_pool = SingleNodeConnectionPool::new(url);
///
/// // load the CA certificate
/// let mut buf = Vec::new();
/// File::open("my_ca_cert.pem")?
///     .read_to_end(&mut buf)?;
/// let cert = Certificate::from_pem(&buf)?;
///
/// let transport = TransportBuilder::new(conn_pool)
///     .cert_validation(CertificateValidation::Full(cert))
///     .build()?;
/// let client = Elasticsearch::new(transport);
/// let _response = client.ping().send().await?;
/// # Ok(())
/// # }
/// ```
pub enum CertificateValidation {
    /// Default validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    ///
    /// A trusted CA is one that is trusted by the operating system on which the client is running,
    /// which typically means that the CA certificate is in the certificate/truststore of the
    /// operating system. This is the default mode of operation.
    Default,
    /// Full validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    ///
    /// This is useful for self-signed certificates generated by your own CA,
    /// where the certificate contains the CommonName (CN) or a Subject Alternative Name (SAN)
    /// that matches the server hostname.
    ///
    /// Typically, the certificate provided to the client is the Certificate Authority (CA)
    /// used to sign the certificate used by the server.
    Full(Certificate),
    /// Validates that the certificate provided by the server is signed by a trusted
    /// Certificate Authority (CA), but does not perform hostname verification.
    ///
    /// This is useful for self-signed certificates generated by your own CA
    /// that **do not** contain the CommonName (CN) or a Subject Alternative Name (SAN)
    /// that matches the server hostname.
    ///
    /// Typically, the certificate provided to the client will be the Certificate Authority (CA)
    /// used to sign the certificate used by the server.
    Certificate(Certificate),
    /// No validation is performed on the certificate provided by the server.
    ///
    /// This disables many of the security benefits of SSL/TLS and should only be used after very
    /// careful consideration. It is primarily intended as a temporary diagnostic mechanism when
    /// attempting to resolve TLS errors, and **its use on production clusters is strongly discouraged**.
    None,
}
