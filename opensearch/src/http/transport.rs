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
//! HTTP transport and connection components

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use crate::auth::ClientCertificate;
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use crate::cert::CertificateValidation;
use crate::{
    auth::Credentials,
    error::Error,
    http::{
        headers::{
            HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE,
            DEFAULT_ACCEPT, DEFAULT_CONTENT_TYPE, DEFAULT_USER_AGENT, USER_AGENT,
        },
        request::Body,
        response::Response,
        Method,
    },
};
use base64::write::EncoderWriter as Base64Encoder;
use bytes::BytesMut;
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    error, fmt,
    fmt::Debug,
    io::{self, Write},
    time::Duration,
};
use url::Url;

/// Error that can occur when building a [Transport]
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
    #[allow(warnings)]
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

/// Default address to Elasticsearch running on `http://localhost:9200`
pub static DEFAULT_ADDRESS: &str = "http://localhost:9200";

lazy_static! {
    /// Client metadata header: service, language, transport, followed by additional information
    static ref CLIENT_META: String = build_meta();
}

fn build_meta() -> String {
    let mut version_parts = env!("CARGO_PKG_VERSION").split(&['.', '-'][..]);
    let mut version = String::new();

    // major.minor.patch followed with an optional 'p' for preliminary versions
    version.push_str(version_parts.next().unwrap());
    version.push('.');
    version.push_str(version_parts.next().unwrap());
    version.push('.');
    version.push_str(version_parts.next().unwrap());
    if version_parts.next().is_some() {
        version.push('p');
    }

    let rustc = env!("RUSTC_VERSION");
    let mut meta = format!("es={},rs={},t={}", version, rustc, version);

    if cfg!(feature = "native-tls") {
        meta.push_str(",tls=n");
    } else if cfg!(feature = "rustls-tls") {
        meta.push_str(",tls=r");
    }

    meta
}

/// Builds a HTTP transport to make API calls to Elasticsearch
pub struct TransportBuilder {
    client_builder: reqwest::ClientBuilder,
    conn_pool: Box<dyn ConnectionPool>,
    credentials: Option<Credentials>,
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    cert_validation: Option<CertificateValidation>,
    proxy: Option<Url>,
    proxy_credentials: Option<Credentials>,
    disable_proxy: bool,
    headers: HeaderMap,
    meta_header: bool,
    timeout: Option<Duration>,
}

impl TransportBuilder {
    /// Creates a new instance of [TransportBuilder]. Accepts a [ConnectionPool]
    /// from which [Connection]s to Elasticsearch will be retrieved.
    pub fn new<P>(conn_pool: P) -> Self
    where
        P: ConnectionPool + Debug + Clone + Send + 'static,
    {
        Self {
            client_builder: reqwest::ClientBuilder::new(),
            conn_pool: Box::new(conn_pool),
            credentials: None,
            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            cert_validation: None,
            proxy: None,
            proxy_credentials: None,
            disable_proxy: false,
            headers: HeaderMap::new(),
            meta_header: true,
            timeout: None,
        }
    }

    /// Configures a proxy.
    ///
    /// An optional username and password will be used to set the
    /// `Proxy-Authorization` header using Basic Authentication.
    pub fn proxy(mut self, url: Url, username: Option<&str>, password: Option<&str>) -> Self {
        self.proxy = Some(url);
        if let Some(u) = username {
            let p = password.unwrap_or("");
            self.proxy_credentials = Some(Credentials::Basic(u.into(), p.into()));
        }

        self
    }

    /// Whether to disable proxies, including system proxies.
    ///
    /// NOTE: System proxies are enabled by default.
    pub fn disable_proxy(mut self) -> Self {
        self.disable_proxy = true;
        self
    }

    /// Credentials for the client to use for authentication to Elasticsearch
    pub fn auth(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Validation applied to the certificate provided to establish a HTTPS connection.
    /// By default, full validation is applied. When using a self-signed certificate,
    /// different validation can be applied.
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    pub fn cert_validation(mut self, validation: CertificateValidation) -> Self {
        self.cert_validation = Some(validation);
        self
    }

    /// Adds a HTTP header that will be added to all client API calls.
    ///
    /// A default HTTP header can be overridden on a per API call basis.
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Adds HTTP headers that will be added to all client API calls.
    ///
    /// Default HTTP headers can be overridden on a per API call basis.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        for (key, value) in headers.iter() {
            self.headers.insert(key, value.clone());
        }
        self
    }

    /// Whether to send a `x-elastic-client-meta` header that describes the runtime environment.
    ///
    /// This header contains information that is similar to what could be found in `User-Agent`. Using a separate
    /// header allows applications to use `User-Agent` for their own needs, e.g. to identify application version
    /// or other environment information. Defaults to `true`.
    pub fn enable_meta_header(mut self, enable: bool) -> Self {
        self.meta_header = enable;
        self
    }

    /// Sets a global request timeout for the client.
    ///
    /// The timeout is applied from when the request starts connecting until the response body has finished.
    /// Default is no timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Builds a [Transport] to use to send API calls to Elasticsearch.
    pub fn build(self) -> Result<Transport, BuildError> {
        let mut client_builder = self.client_builder;

        if !self.headers.is_empty() {
            client_builder = client_builder.default_headers(self.headers);
        }

        if let Some(t) = self.timeout {
            client_builder = client_builder.timeout(t);
        }

        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        {
            if let Some(creds) = &self.credentials {
                if let Credentials::Certificate(cert) = creds {
                    client_builder = match cert {
                        #[cfg(feature = "native-tls")]
                        ClientCertificate::Pkcs12(b, p) => {
                            let password = match p {
                                Some(pass) => pass.as_str(),
                                None => "",
                            };
                            let pkcs12 = reqwest::Identity::from_pkcs12_der(b, password)?;
                            client_builder.identity(pkcs12)
                        }
                        #[cfg(feature = "rustls-tls")]
                        ClientCertificate::Pem(b) => {
                            let pem = reqwest::Identity::from_pem(b)?;
                            client_builder.identity(pem)
                        }
                    }
                }
            };
        }

        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        if let Some(v) = self.cert_validation {
            client_builder = match v {
                CertificateValidation::Default => client_builder,
                CertificateValidation::Full(chain) => {
                    chain.into_iter().fold(client_builder, |client_builder, c| {
                        client_builder.add_root_certificate(c)
                    })
                }
                #[cfg(feature = "native-tls")]
                CertificateValidation::Certificate(chain) => chain
                    .into_iter()
                    .fold(client_builder, |client_builder, c| {
                        client_builder.add_root_certificate(c)
                    })
                    .danger_accept_invalid_hostnames(true),
                CertificateValidation::None => client_builder.danger_accept_invalid_certs(true),
            }
        }

        if self.disable_proxy {
            client_builder = client_builder.no_proxy();
        } else if let Some(url) = self.proxy {
            let mut proxy = reqwest::Proxy::all(url)?;
            if let Some(c) = self.proxy_credentials {
                proxy = match c {
                    Credentials::Basic(u, p) => proxy.basic_auth(&u, &p),
                    _ => proxy,
                };
            }
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build()?;
        Ok(Transport {
            client,
            conn_pool: self.conn_pool,
            credentials: self.credentials,
            send_meta: self.meta_header,
        })
    }
}

impl Default for TransportBuilder {
    /// Creates a default implementation using the default implementation of [SingleNodeConnectionPool].
    fn default() -> Self {
        Self::new(SingleNodeConnectionPool::default())
    }
}

/// A connection to an Elasticsearch node, used to send an API request
#[derive(Debug, Clone)]
pub struct Connection {
    url: Url,
}

impl Connection {
    /// Creates a new instance of a [Connection].
    ///
    /// If the passed [url::Url] path does not have a trailing forward slash, a trailing
    /// forward slash will be appended
    pub fn new(url: Url) -> Self {
        let mut url = url;
        if !url.path().ends_with('/') {
            url.set_path(&format!("{}/", url.path()));
        }

        Self { url }
    }
}

/// A HTTP transport responsible for making the API requests to Elasticsearch,
/// using a [Connection] selected from a [ConnectionPool]
#[derive(Debug, Clone)]
pub struct Transport {
    client: reqwest::Client,
    credentials: Option<Credentials>,
    conn_pool: Box<dyn ConnectionPool>,
    send_meta: bool,
}

impl Transport {
    fn method(&self, method: Method) -> reqwest::Method {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Put => reqwest::Method::PUT,
            Method::Post => reqwest::Method::POST,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
        }
    }

    fn bytes_mut(&self) -> BytesMut {
        // NOTE: These could be pooled or re-used
        BytesMut::with_capacity(1024)
    }

    /// Creates a new instance of a [Transport] configured with a
    /// [SingleNodeConnectionPool].
    pub fn single_node(url: &str) -> Result<Transport, Error> {
        let u = Url::parse(url)?;
        let conn_pool = SingleNodeConnectionPool::new(u);
        let transport = TransportBuilder::new(conn_pool).build()?;
        Ok(transport)
    }

    /// Creates a new instance of a [Transport] configured for use with
    /// [Elasticsearch service in Elastic Cloud](https://www.elastic.co/cloud/).
    ///
    /// * `cloud_id`: The Elastic Cloud Id retrieved from the cloud web console, that uniquely
    ///               identifies the deployment instance.
    /// * `credentials`: A set of credentials the client should use to authenticate to Elasticsearch service.
    pub fn cloud(cloud_id: &str, credentials: Credentials) -> Result<Transport, Error> {
        let conn_pool = CloudConnectionPool::new(cloud_id)?;
        let transport = TransportBuilder::new(conn_pool).auth(credentials).build()?;
        Ok(transport)
    }

    /// Creates an asynchronous request that can be awaited
    pub async fn send<B, Q>(
        &self,
        method: Method,
        path: &str,
        headers: HeaderMap,
        query_string: Option<&Q>,
        body: Option<B>,
        timeout: Option<Duration>,
    ) -> Result<Response, Error>
    where
        B: Body,
        Q: Serialize + ?Sized,
    {
        let connection = self.conn_pool.next();
        let url = connection.url.join(path.trim_start_matches('/'))?;
        let reqwest_method = self.method(method);
        let mut request_builder = self.client.request(reqwest_method, url);

        if let Some(t) = timeout {
            request_builder = request_builder.timeout(t);
        }

        // set credentials before any headers, as credentials append to existing headers in reqwest,
        // whilst setting headers() overwrites, so if an Authorization header has been specified
        // on a specific request, we want it to overwrite.
        if let Some(c) = &self.credentials {
            request_builder = match c {
                Credentials::Basic(u, p) => request_builder.basic_auth(u, Some(p)),
                Credentials::Bearer(t) => request_builder.bearer_auth(t),
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                Credentials::Certificate(_) => request_builder,
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

        // default headers first, overwrite with any provided
        let mut request_headers = HeaderMap::with_capacity(4 + headers.len());
        request_headers.insert(CONTENT_TYPE, HeaderValue::from_static(DEFAULT_CONTENT_TYPE));
        request_headers.insert(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT));
        request_headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        for (name, value) in headers {
            request_headers.insert(name.unwrap(), value);
        }
        // if meta header enabled, send it last so that it's not overridden.
        if self.send_meta {
            request_headers.insert(
                "x-elastic-client-meta",
                HeaderValue::from_static(CLIENT_META.as_str()),
            );
        }

        request_builder = request_builder.headers(request_headers);

        if let Some(b) = body {
            let bytes = if let Some(bytes) = b.bytes() {
                bytes
            } else {
                let mut bytes_mut = self.bytes_mut();
                b.write(&mut bytes_mut)?;
                bytes_mut.split().freeze()
            };

            request_builder = request_builder.body(bytes);
        };

        if let Some(q) = query_string {
            request_builder = request_builder.query(q);
        }

        let response = request_builder.send().await;
        match response {
            Ok(r) => Ok(Response::new(r, method)),
            Err(e) => Err(e.into()),
        }
    }
}

impl Default for Transport {
    fn default() -> Self {
        TransportBuilder::default().build().unwrap()
    }
}

/// A pool of [Connection]s, used to make API calls to Elasticsearch.
///
/// A [ConnectionPool] manages the connections, with different implementations determining how
/// to get the next [Connection]. The simplest type of [ConnectionPool] is [SingleNodeConnectionPool],
/// which manages only a single connection, but other implementations may manage connections more
/// dynamically at runtime, based upon the response to API calls.
pub trait ConnectionPool: Debug + dyn_clone::DynClone + Sync + Send {
    /// Gets a reference to the next [Connection]
    fn next(&self) -> &Connection;
}

clone_trait_object!(ConnectionPool);

/// A connection pool that manages the single connection to an Elasticsearch cluster.
#[derive(Debug, Clone)]
pub struct SingleNodeConnectionPool {
    connection: Connection,
}

impl SingleNodeConnectionPool {
    /// Creates a new instance of [SingleNodeConnectionPool].
    pub fn new(url: Url) -> Self {
        Self {
            connection: Connection::new(url),
        }
    }
}

impl Default for SingleNodeConnectionPool {
    /// Creates a default instance of [SingleNodeConnectionPool], using [DEFAULT_ADDRESS].
    fn default() -> Self {
        Self::new(Url::parse(DEFAULT_ADDRESS).unwrap())
    }
}

impl ConnectionPool for SingleNodeConnectionPool {
    /// Gets a reference to the next [Connection]
    fn next(&self) -> &Connection {
        &self.connection
    }
}

/// An identifier that uniquely identifies an Elasticsearch cluster running
/// on [Elasticsearch service in Elastic Cloud](https://www.elastic.co/cloud/).
///
/// See the [Cloud Id documentation](https://www.elastic.co/guide/en/cloud/current/ec-cloud-id.html)
/// for more details.
#[derive(Debug, Clone)]
pub struct CloudId {
    /// The name of the cluster
    pub name: String,
    /// The [url::Url] to the cluster
    pub url: Url,
}

impl CloudId {
    /// Parses a [CloudId] from a string slice representation of the id retrieved from the cloud
    /// web console
    pub fn parse(cloud_id: &str) -> Result<CloudId, Error> {
        if cloud_id.is_empty() || !cloud_id.contains(':') {
            return Err(crate::error::lib(
                "cloud_id should be of the form '<cluster name>:<base64 data>'",
            ));
        }

        let parts: Vec<&str> = cloud_id.splitn(2, ':').collect();
        let name: String = parts[0].into();
        if name.is_empty() {
            return Err(crate::error::lib("cloud_id cluster name cannot be empty"));
        }

        let data = parts[1];
        let decoded_result = base64::decode(data);
        if decoded_result.is_err() {
            return Err(crate::error::lib(format!(
                "cannot base 64 decode '{}'",
                data
            )));
        }

        let decoded = decoded_result.unwrap();
        let mut decoded_parts = decoded.split(|&b| b == b'$');
        let domain_name;
        let uuid;

        match decoded_parts.next() {
            Some(p) => {
                match std::str::from_utf8(p) {
                    Ok(s) => {
                        domain_name = s.trim();
                        if domain_name.is_empty() {
                            return Err(crate::error::lib("decoded '<base64 data>' must contain a domain name as the first part"));
                        }
                    }
                    Err(_) => {
                        return Err(crate::error::lib(
                            "decoded '<base64 data>' must contain a domain name as the first part",
                        ))
                    }
                }
            }
            None => {
                return Err(crate::error::lib(
                    "decoded '<base64 data>' must contain a domain name as the first part",
                ));
            }
        }

        match decoded_parts.next() {
            Some(p) => match std::str::from_utf8(p) {
                Ok(s) => {
                    uuid = s.trim();
                    if uuid.is_empty() {
                        return Err(crate::error::lib(
                            "decoded '<base64 data>' must contain a uuid as the second part",
                        ));
                    }
                }
                Err(_) => {
                    return Err(crate::error::lib(
                        "decoded '<base64 data>' must contain a uuid as the second part",
                    ))
                }
            },
            None => {
                return Err(crate::error::lib(
                    "decoded '<base64 data>' must contain at least two parts",
                ));
            }
        }

        let url = Url::parse(format!("https://{}.{}", uuid, domain_name).as_ref())?;
        Ok(CloudId { name, url })
    }
}

/// A connection pool that manages the single connection to an Elasticsearch cluster running
/// on [Elasticsearch service in Elastic Cloud](https://www.elastic.co/cloud/).
#[derive(Debug, Clone)]
pub struct CloudConnectionPool {
    cloud_id: CloudId,
    connection: Connection,
}

impl CloudConnectionPool {
    /// Creates a new instance of [CloudConnectionPool].
    pub fn new(cloud_id: &str) -> Result<Self, Error> {
        let cloud = CloudId::parse(cloud_id)?;
        let connection = Connection::new(cloud.url.clone());
        Ok(Self {
            cloud_id: cloud,
            connection,
        })
    }
}

impl ConnectionPool for CloudConnectionPool {
    /// Gets a reference to the next [Connection]
    fn next(&self) -> &Connection {
        &self.connection
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    use crate::auth::ClientCertificate;
    use regex::Regex;
    use url::Url;

    #[test]
    #[cfg(feature = "native-tls")]
    fn invalid_pkcs12_cert_credentials() {
        let conn_pool = SingleNodeConnectionPool::default();
        let builder = TransportBuilder::new(conn_pool)
            .auth(ClientCertificate::Pkcs12(b"Nonsense".to_vec(), None).into());

        let res = builder.build();
        assert!(res.is_err());
    }

    #[test]
    #[cfg(feature = "rustls-tls")]
    fn invalid_pem_cert_credentials() {
        let conn_pool = SingleNodeConnectionPool::default();
        let builder = TransportBuilder::new(conn_pool)
            .auth(ClientCertificate::Pem(b"Nonsense".to_vec()).into());

        let res = builder.build();
        assert!(res.is_err());
    }

    #[test]
    fn can_parse_cloud_id_with_kibana_uuid() {
        let base64 = base64::encode("cloud-endpoint.example$3dadf823f05388497ea684236d918a1a$3f26e1609cf54a0f80137a80de560da4");
        let cloud_id = format!("my_cluster:{}", base64);
        let result = CloudId::parse(&cloud_id);
        assert!(result.is_ok());
        let cloud = result.unwrap();
        assert_eq!("my_cluster", cloud.name);
        assert_eq!(
            Url::parse("https://3dadf823f05388497ea684236d918a1a.cloud-endpoint.example").unwrap(),
            cloud.url
        );
    }

    #[test]
    fn can_parse_cloud_id_without_kibana_uuid() {
        let base64 = base64::encode("cloud-endpoint.example$3dadf823f05388497ea684236d918a1a$");
        let cloud_id = format!("my_cluster:{}", base64);
        let result = CloudId::parse(&cloud_id);
        assert!(result.is_ok());
        let cloud = result.unwrap();
        assert_eq!("my_cluster", cloud.name);
        assert_eq!(
            Url::parse("https://3dadf823f05388497ea684236d918a1a.cloud-endpoint.example").unwrap(),
            cloud.url
        );
    }

    #[test]
    fn can_parse_cloud_id_with_different_port() {
        let base64 = base64::encode("cloud-endpoint.example:4463$3dadf823f05388497ea684236d918a1a$3f26e1609cf54a0f80137a80de560da4");
        let cloud_id = format!("my_cluster:{}", base64);
        let result = CloudId::parse(&cloud_id);
        assert!(result.is_ok());
        let cloud = result.unwrap();
        assert_eq!("my_cluster", cloud.name);
        assert_eq!(
            Url::parse("https://3dadf823f05388497ea684236d918a1a.cloud-endpoint.example:4463")
                .unwrap(),
            cloud.url
        );
    }

    #[test]
    fn cloud_id_must_contain_colon() {
        let base64 = base64::encode("cloud-endpoint.example$3dadf823f05388497ea684236d918a1a$3f26e1609cf54a0f80137a80de560da4");
        let cloud_id = format!("my_cluster{}", base64);
        let cloud = CloudId::parse(&cloud_id);
        assert!(cloud.is_err());
    }

    #[test]
    fn cloud_id_second_part_must_be_base64() {
        let cloud_id = "my_cluster:not_base_64";
        let cloud = CloudId::parse(cloud_id);
        assert!(cloud.is_err());
    }

    #[test]
    fn cloud_id_first_part_cannot_be_empty() {
        let base64 = base64::encode("cloud-endpoint.example$3dadf823f05388497ea684236d918a1a$3f26e1609cf54a0f80137a80de560da4");
        let cloud_id = format!(":{}", base64);
        let cloud = CloudId::parse(&cloud_id);
        assert!(cloud.is_err());
    }

    #[test]
    fn cloud_id_second_part_cannot_be_empty() {
        let cloud_id = "cluster_name:";
        let cloud = CloudId::parse(&cloud_id);
        assert!(cloud.is_err());
    }

    #[test]
    fn cloud_id_second_part_must_have_at_least_two_parts() {
        let base64 = base64::encode("cloud-endpoint.example");
        let cloud_id = format!("my_cluster:{}", base64);
        let result = CloudId::parse(&cloud_id);
        assert!(result.is_err());
    }

    #[test]
    fn connection_url_with_no_trailing_slash() {
        let url = Url::parse("http://10.1.2.3/path_with_no_trailing_slash").unwrap();
        let conn = Connection::new(url);
        assert_eq!(
            conn.url.as_str(),
            "http://10.1.2.3/path_with_no_trailing_slash/"
        );
    }

    #[test]
    fn connection_url_with_trailing_slash() {
        let url = Url::parse("http://10.1.2.3/path_with_trailing_slash/").unwrap();
        let conn = Connection::new(url);
        assert_eq!(
            conn.url.as_str(),
            "http://10.1.2.3/path_with_trailing_slash/"
        );
    }

    #[test]
    fn connection_url_with_no_path_and_no_trailing_slash() {
        let url = Url::parse("http://10.1.2.3").unwrap();
        let conn = Connection::new(url);
        assert_eq!(conn.url.as_str(), "http://10.1.2.3/");
    }

    #[test]
    fn connection_url_with_no_path_and_trailing_slash() {
        let url = Url::parse("http://10.1.2.3/").unwrap();
        let conn = Connection::new(url);
        assert_eq!(conn.url.as_str(), "http://10.1.2.3/");
    }

    #[test]
    pub fn test_meta_header() {
        let re = Regex::new(r"^es=[0-9]{1,2}\.[0-9]{1,2}\.[0-9]{1,3}p?,rs=[0-9]{1,2}\.[0-9]{1,2}\.[0-9]{1,3}p?,t=[0-9]{1,2}\.[0-9]{1,2}\.[0-9]{1,3}p?(,tls=[rn])?$").unwrap();
        let x: &str = CLIENT_META.as_str();
        println!("{}", x);
        assert!(re.is_match(x));
    }
}
