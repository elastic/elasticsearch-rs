use crate::{
    auth::Credentials,
    error::Error,
    http::{
        headers::{DEFAULT_ACCEPT, DEFAULT_CONTENT_TYPE, DEFAULT_USER_AGENT, HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
        request::Body,
        response::Response,
        Method,
    },
};

use base64;
use base64::write::EncoderWriter as Base64Encoder;
use bytes::BytesMut;
use serde::Serialize;
use std::error;
use std::fmt;
use std::io::{self, Write};
use url::Url;

/// Error that can occur when building a connection
#[derive(Debug)]
pub enum BuildError {
    /// IO error
    Io(io::Error),

    /// Certificate error
    Cert(reqwest::Error),
}

impl From<io::Error> for BuildError {
    fn from(err: io::Error) -> BuildError {
        BuildError::Io(err)
    }
}

impl From<reqwest::Error> for BuildError {
    fn from(err: reqwest::Error) -> BuildError {
        BuildError::Cert(err)
    }
}

impl error::Error for BuildError {
    fn description(&self) -> &str {
        match *self {
            BuildError::Io(ref err) => err.description(),
            BuildError::Cert(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            BuildError::Io(ref err) => Some(err as &dyn error::Error),
            BuildError::Cert(ref err) => Some(err as &dyn error::Error),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::Io(ref err) => fmt::Display::fmt(err, f),
            BuildError::Cert(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

/// The default address to Elasticsearch.
pub static DEFAULT_ADDRESS: &str = "http://localhost:9200";

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
    fn method(&self, method: Method) -> reqwest::Method {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Put => reqwest::Method::PUT,
            Method::Post => reqwest::Method::POST,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
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
        method: Method,
        path: &str,
        headers: HeaderMap,
        query_string: Option<&Q>,
        body: Option<B>,
    ) -> Result<Response, Error>
    where
        B: Body,
        Q: Serialize + ?Sized,
    {
        let url = self.url.join(path)?;
        let reqwest_method = self.method(method);
        let mut request_builder = self.client.request(reqwest_method, &url.to_string());

        let mut headers = headers;
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(DEFAULT_CONTENT_TYPE));
        headers.insert(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT));
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        request_builder = request_builder.headers(headers);

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
        Ok(Response::new(response))
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new(Url::parse(DEFAULT_ADDRESS).unwrap())
    }
}
