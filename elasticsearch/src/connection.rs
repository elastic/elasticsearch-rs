use crate::{
    error::ElasticsearchError, http_method::HttpMethod, response::ElasticsearchResponse, Body,
};

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use base64;
use base64::write::EncoderWriter as Base64Encoder;
use bytes::{BufMut, Bytes, BytesMut};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client, Method,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

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

/// Error that can occur when building a connection
#[derive(Debug)]
pub enum BuildError {
    /// IO error
    IoError(io::Error),

    /// Certificate error
    CertError(reqwest::Error),
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> BuildError {
        BuildError::IoError(err)
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(err: reqwest::Error) -> BuildError {
        BuildError::CertError(err)
    }
}

impl Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::IoError(ref err) => err.description(),
            BuildError::CertError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            BuildError::IoError(ref err) => Some(err as &dyn Error),
            BuildError::CertError(ref err) => Some(err as &dyn Error),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::IoError(ref err) => fmt::Display::fmt(err, f),
            BuildError::CertError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

pub static DEFAULT_ADDRESS: &str = "http://localhost:9200";

static DEFAULT_USER_AGENT: &str = concat!("elasticsearch-rs/", env!("CARGO_PKG_VERSION"));

static DEFAULT_CONTENT_TYPE: &str = "application/json";

static DEFAULT_ACCEPT: &str = "application/json";

pub struct ConnectionBuilder {
    client_builder: reqwest::ClientBuilder,
    url: Url,
    credentials: Option<Credentials>,
    proxy: Option<Url>,
    disable_proxy: bool,
}

impl ConnectionBuilder {
    pub fn new(url: Url) -> Self {
        ConnectionBuilder {
            client_builder: reqwest::ClientBuilder::new(),
            url,
            credentials: None,
            proxy: None,
            disable_proxy: false,
        }
    }

    /// Configures a proxy
    pub fn proxy(mut self, url: Url) -> Self {
        self.proxy = Some(url);
        self
    }

    /// Whether to disable proxies, including system proxies
    pub fn disable_proxy(mut self) -> Self {
        self.disable_proxy = true;
        self
    }

    pub fn auth(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Builds a connection to an Elasticsearch node
    pub fn build(self) -> Result<Connection, BuildError> {
        let mut client_builder = self.client_builder;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(DEFAULT_CONTENT_TYPE));
        headers.insert(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT));
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        client_builder = client_builder.default_headers(headers);

        if let Some(c) = &self.credentials {
            client_builder = match c {
                Credentials::Cert(b, p) => {
                    let pkcs12 = reqwest::Identity::from_pkcs12_der(&b, p)?;
                    client_builder.identity(pkcs12)
                }
                _ => client_builder,
            }
        };

        if self.disable_proxy {
            client_builder = client_builder.no_proxy();
        } else if let Some(url) = &self.proxy {
            client_builder = match url.scheme() {
                "https" => client_builder.proxy(reqwest::Proxy::https(&url.to_string())?),
                _ => client_builder.proxy(reqwest::Proxy::http(&url.to_string())?),
            };
        }

        let client = client_builder.build()?;
        Ok(Connection {
            url: self.url,
            credentials: self.credentials,
            client,
        })
    }
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        ConnectionBuilder::new(Url::parse(DEFAULT_ADDRESS).unwrap())
    }
}

/// A connection to an Elasticsearch node, used to send an API request
#[derive(Debug, Clone)]
pub struct Connection {
    client: reqwest::Client,
    url: Url,
    credentials: Option<Credentials>,
}

impl Connection {
    fn method(&self, method: HttpMethod) -> Method {
        match method {
            HttpMethod::Get => Method::GET,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Post => Method::POST,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Head => Method::HEAD,
        }
    }

    pub fn new(url: Url) -> Connection {
        Connection {
            client: reqwest::Client::new(),
            url,
            credentials: None,
        }
    }

    /// Creates an asynchronous request that can be awaited
    pub async fn send<B, Q>(
        &self,
        method: HttpMethod,
        path: &str,
        query_string: Option<&Q>,
        body: Option<B>,
    ) -> Result<ElasticsearchResponse, ElasticsearchError>
    where
        B: Body,
        Q: Serialize + ?Sized,
    {
        let url = self.url.join(path)?;
        let reqwest_method = self.method(method);
        let mut request_builder = self.client.request(reqwest_method, &url.to_string());

        if let Some(b) = body {
            let mut bytes_mut = BytesMut::with_capacity(1024);
            b.write(&mut bytes_mut)?;
            let bytes = bytes_mut.freeze();
            // TODO: pass Bytes directly once reqwest is updated to Bytes ^0.5 crate
            request_builder = request_builder.body(bytes.to_vec());
        };

        if let Some(q) = query_string {
            request_builder = request_builder.query(q);
        }

        if let Some(c) = &self.credentials {
            request_builder = match c {
                Credentials::Basic(u, p) => request_builder.basic_auth(u, Some(p)),
                Credentials::Bearer(t) => request_builder.bearer_auth(t),
                Credentials::Cert(_, _) => request_builder,
                Credentials::ApiKey(i, k) => {
                    let mut header_value = b"ApiKey ".to_vec();
                    {
                        let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
                        write!(encoder, "{}:", i).unwrap();
                        write!(encoder, "{}", k).unwrap();
                    }
                    request_builder.header(
                        AUTHORIZATION,
                        HeaderValue::from_bytes(&header_value).unwrap(),
                    )
                }
            }
        }

        let response = request_builder.send().await?;
        Ok(ElasticsearchResponse::new(response))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse(DEFAULT_ADDRESS).unwrap())
    }
}
