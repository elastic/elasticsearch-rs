use crate::{error::ElasticsearchError, http_method::HttpMethod, response::ElasticsearchResponse};

use std::fs::File;
use std::io::Read;
use std::io;
use std::io::Write;
use std::error::Error;
use std::fmt;

use base64;
use base64::write::EncoderWriter as Base64Encoder;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client, Method,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub enum Credentials {
    /// A username and password to use for Basic authentication
    Basic(String, String),
    /// A token to use for Bearer authentication
    Bearer(String),
    /// A path to a PKCS#12 archive and password to use for Client Certification authentication
    Certificate(String, String),
    /// An id an api_key to use for API key authentication
    ApiKey(String, String),
}

/// Error that can occur when building a connection
#[derive(Debug)]
pub enum BuildError {
    /// IO error
    IoError(io::Error),

    /// Certificate error
    CertificateError(reqwest::Error),
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> BuildError {
        BuildError::IoError(err)
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(err: reqwest::Error) -> BuildError {
        BuildError::CertificateError(err)
    }
}

impl Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::IoError(ref err) => err.description(),
            BuildError::CertificateError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            BuildError::IoError(ref err) => Some(err as &dyn Error),
            BuildError::CertificateError(ref err) => Some(err as &dyn Error),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::IoError(ref err) => fmt::Display::fmt(err, f),
            BuildError::CertificateError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

pub struct ConnectionBuilder {
    client_builder: reqwest::ClientBuilder,
    url: Url,
    credentials: Option<Credentials>,
    proxy: Option<Url>,
}

impl ConnectionBuilder {
    pub fn new(url: Url) -> Self {
        ConnectionBuilder {
            client_builder: reqwest::ClientBuilder::new(),
            url,
            credentials: None,
            proxy: None,
        }
    }

    pub fn proxy(mut self, url: Url) -> Self {
        self.proxy = Some(url);
        self
    }

    pub fn auth(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Builds a connection to an Elasticsearch node
    pub fn build(self) -> Result<Connection, BuildError> {
        let mut client_builder = self.client_builder;
        if let Some(c) = &self.credentials {
            match c {
                Credentials::Certificate(f, p) => {
                    let mut buf = Vec::new();
                    File::open(f)?
                        .read_to_end(&mut buf)?;
                    let pkcs12 = reqwest::Identity::from_pkcs12_der(&buf, p)?;
                    client_builder = client_builder.identity(pkcs12);
                },
                _ => (),
            }
        };

        if let Some(url) = &self.proxy {
            client_builder = match url.scheme() {
                "https" => client_builder.proxy(reqwest::Proxy::https(&url.to_string())?),
                _ => client_builder.proxy(reqwest::Proxy::http(&url.to_string())?),
            };
        }

        let client = client_builder.build()?;
        Ok(Connection {
            url: self.url,
            credentials: self.credentials,
            client
        })
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
        B: Serialize,
        Q: Serialize + ?Sized,
    {
        let url = self.url.join(path)?;
        let reqwest_method = self.method(method);
        let mut request_builder = self.client.request(reqwest_method, &url.to_string());
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        // TODO: autogenerate user agent with version, based on version in Cargo.toml, which will align with the REST spec version of Elasticsearch
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("elasticsearch-rs/0.1.0"),
        );

        request_builder = request_builder.headers(headers);

        if let Some(b) = body {
            request_builder = request_builder.json(&b);
        };

        if let Some(q) = query_string {
            request_builder = request_builder.query(q);
        }

        if let Some(c) = &self.credentials {
            request_builder = match c {
                Credentials::Basic(u, p) => request_builder.basic_auth(u, Some(p)),
                Credentials::Bearer(t) => request_builder.bearer_auth(t),
                Credentials::Certificate(_, _) => request_builder,
                Credentials::ApiKey(i, k) => {
                    let mut header_value = b"ApiKey ".to_vec();
                    {
                        let mut encoder = Base64Encoder::new(&mut header_value, base64::STANDARD);
                        write!(encoder, "{}:", i).unwrap();
                        write!(encoder, "{}", k).unwrap();
                    }
                    request_builder.header(AUTHORIZATION, HeaderValue::from_bytes(&header_value).unwrap())
                },
            }
        }

        let response = request_builder.send().await?;
        Ok(ElasticsearchResponse::new(response))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse("http://localhost:9200").unwrap())
    }
}
