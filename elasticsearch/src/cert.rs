//! X.509 certificate components
pub use reqwest::Certificate;

/// Validation applied to the certificate to establish a HTTPS connection
pub enum CertificateValidation {
    /// Default validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    Default,
    /// Full validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    ///
    /// Typically, the certificate provided to the client will the Certificate Authority (CA)
    /// used to generated the certificate.
    ///
    /// This is useful for self-signed certificates where the certificate contains the CommonName (CN)
    /// or a Subject Alternative Name (SAN) that matches the server hostname.
    Full(Certificate),
    /// Validates that the certificate provided by the server is signed by a trusted
    /// Certificate Authority (CA), but does not perform hostname verification.
    ///
    /// Typically, the certificate provided to the client will be the Certificate Authority (CA)
    /// used to generated the certificate.
    ///
    /// This is useful for self-signed certificates that **do not** contain the CommonName (CN)
    /// or a Subject Alternative Name (SAN) that matches the server hostname.
    Certificate(Certificate),
    /// No validation is performed the certificate provided by the server. This disables many of the
    /// security benefits of SSL/TLS and should only be used after very careful consideration.
    /// It is primarily intended as a temporary diagnostic mechanism when attempting to resolve
    /// TLS errors, and **its use on production clusters is strongly discouraged**.
    None,
}
