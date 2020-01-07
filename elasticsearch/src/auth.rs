//! Authentication components

/// Credentials for authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// A username and password to use for Basic authentication
    Basic(String, String),
    /// An access_token to use for Bearer authentication
    Bearer(String),
    /// Bytes of a DER-formatted PKCS#12 archive and password to use for
    /// PKI (Client Certificate) authentication.
    ///
    /// The archive should contain a leaf certificate and its private key, as well any intermediate
    /// certificates that allow clients to build a chain to a trusted root. The chain certificates
    /// should be in order from the leaf certificate towards the root.
    Cert(Vec<u8>, String),
    /// An id and api_key to use for API key authentication
    ApiKey(String, String),
}
