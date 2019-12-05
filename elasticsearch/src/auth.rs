/// Credentials for authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// A username and password to use for Basic authentication
    Basic(String, String),
    /// An access_token to use for Bearer authentication
    Bearer(String),
    /// Bytes of a PKCS#12 archive and password to use for PKI (Client Certificate) authentication
    Cert(Vec<u8>, String),
    /// An id an api_key to use for API key authentication
    ApiKey(String, String),
}
