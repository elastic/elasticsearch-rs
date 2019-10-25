// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p api_generator
//
// -----------------------------------------------
use super::super::client::Elasticsearch;
use super::super::enums::*;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct CatAliases {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    name: Option<Vec<String>>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatAliases {
    pub fn new(client: Elasticsearch) -> Self {
        CatAliases {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatAliases {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatAllocation {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    node_id: Option<Vec<String>>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatAllocation {
    pub fn new(client: Elasticsearch) -> Self {
        CatAllocation {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatAllocation {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatCount {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatCount {
    pub fn new(client: Elasticsearch) -> Self {
        CatCount {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatCount {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatFielddata {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    fields: Option<Vec<String>>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatFielddata {
    pub fn new(client: Elasticsearch) -> Self {
        CatFielddata {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of fields to return in the output"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatFielddata {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatHealth {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    ts: Option<bool>,
    v: Option<bool>,
}
impl CatHealth {
    pub fn new(client: Elasticsearch) -> Self {
        CatHealth {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Set to false to disable timestamping"]
    pub fn ts(mut self, ts: Option<bool>) -> Self {
        self.ts = ts;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatHealth {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatHelp {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
}
impl CatHelp {
    pub fn new(client: Elasticsearch) -> Self {
        CatHelp {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
}
impl Sender for CatHelp {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatIndices {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    health: Option<Health>,
    help: Option<bool>,
    human: Option<bool>,
    include_unloaded_segments: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    pri: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatIndices {
    pub fn new(client: Elasticsearch) -> Self {
        CatIndices {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
    pub fn health(mut self, health: Option<Health>) -> Self {
        self.health = health;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: Option<bool>) -> Self {
        self.include_unloaded_segments = include_unloaded_segments;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Set to true to return stats only for primary shards"]
    pub fn pri(mut self, pri: Option<bool>) -> Self {
        self.pri = pri;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatIndices {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatMaster {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatMaster {
    pub fn new(client: Elasticsearch) -> Self {
        CatMaster {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatMaster {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatNodeattrs {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatNodeattrs {
    pub fn new(client: Elasticsearch) -> Self {
        CatNodeattrs {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatNodeattrs {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatNodes {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    full_id: Option<bool>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatNodes {
    pub fn new(client: Elasticsearch) -> Self {
        CatNodes {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Return the full node ID instead of the shortened version (default: false)"]
    pub fn full_id(mut self, full_id: Option<bool>) -> Self {
        self.full_id = full_id;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatNodes {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatPendingTasks {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatPendingTasks {
    pub fn new(client: Elasticsearch) -> Self {
        CatPendingTasks {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatPendingTasks {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatPlugins {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatPlugins {
    pub fn new(client: Elasticsearch) -> Self {
        CatPlugins {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatPlugins {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatRecovery {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatRecovery {
    pub fn new(client: Elasticsearch) -> Self {
        CatRecovery {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatRecovery {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatRepositories {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatRepositories {
    pub fn new(client: Elasticsearch) -> Self {
        CatRepositories {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatRepositories {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatSegments {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatSegments {
    pub fn new(client: Elasticsearch) -> Self {
        CatSegments {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatSegments {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatShards {
    client: Elasticsearch,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<Vec<String>>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatShards {
    pub fn new(client: Elasticsearch) -> Self {
        CatShards {
            client,
            ..Default::default()
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<Bytes>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatShards {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatSnapshots {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    repository: Option<Vec<String>>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatSnapshots {
    pub fn new(client: Elasticsearch) -> Self {
        CatSnapshots {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Set to true to ignore unavailable snapshots"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: Option<bool>) -> Self {
        self.ignore_unavailable = ignore_unavailable;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatSnapshots {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatTasks {
    client: Elasticsearch,
    actions: Option<Vec<String>>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    node_id: Option<Vec<String>>,
    parent_task: Option<i64>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatTasks {
    pub fn new(client: Elasticsearch) -> Self {
        CatTasks {
            client,
            ..Default::default()
        }
    }
    #[doc = "A comma-separated list of actions that should be returned. Leave empty to return all."]
    pub fn actions(mut self, actions: Option<Vec<String>>) -> Self {
        self.actions = actions;
        self
    }
    #[doc = "Return detailed task information (default: false)"]
    pub fn detailed(mut self, detailed: Option<bool>) -> Self {
        self.detailed = detailed;
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: Option<Vec<String>>) -> Self {
        self.node_id = node_id;
        self
    }
    #[doc = "Return tasks with specified parent task id. Set to -1 to return all."]
    pub fn parent_task(mut self, parent_task: Option<i64>) -> Self {
        self.parent_task = parent_task;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatTasks {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatTemplates {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    name: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    source: Option<String>,
    v: Option<bool>,
}
impl CatTemplates {
    pub fn new(client: Elasticsearch) -> Self {
        CatTemplates {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatTemplates {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[derive(Default)]
pub struct CatThreadPool {
    client: Elasticsearch,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pretty: Option<bool>,
    s: Option<Vec<String>>,
    size: Option<Size>,
    source: Option<String>,
    thread_pool_patterns: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatThreadPool {
    pub fn new(client: Elasticsearch) -> Self {
        CatThreadPool {
            client,
            ..Default::default()
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: Option<bool>) -> Self {
        self.error_trace = error_trace;
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: Option<Vec<String>>) -> Self {
        self.filter_path = filter_path;
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: Option<String>) -> Self {
        self.format = format;
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: Option<Vec<String>>) -> Self {
        self.h = h;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: Option<bool>) -> Self {
        self.human = human;
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: Option<bool>) -> Self {
        self.local = local;
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: Option<bool>) -> Self {
        self.pretty = pretty;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The multiplier in which to display values"]
    pub fn size(mut self, size: Option<Size>) -> Self {
        self.size = size;
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatThreadPool {
    fn send<T>(self) -> Result<ElasticsearchResponse<T>>
    where
        T: DeserializeOwned,
    {
        Ok(ElasticsearchResponse {
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            body: None,
        })
    }
}
#[doc = "Cat APIs"]
pub struct Cat {
    client: Elasticsearch,
}
impl Cat {
    pub fn new(client: Elasticsearch) -> Self {
        Cat { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-alias.html"]
    pub fn aliases(&self) -> CatAliases {
        CatAliases::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html"]
    pub fn allocation(&self) -> CatAllocation {
        CatAllocation::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html"]
    pub fn count(&self) -> CatCount {
        CatCount::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html"]
    pub fn fielddata(&self) -> CatFielddata {
        CatFielddata::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html"]
    pub fn health(&self) -> CatHealth {
        CatHealth::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html"]
    pub fn help(&self) -> CatHelp {
        CatHelp::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html"]
    pub fn indices(&self) -> CatIndices {
        CatIndices::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html"]
    pub fn master(&self) -> CatMaster {
        CatMaster::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html"]
    pub fn nodeattrs(&self) -> CatNodeattrs {
        CatNodeattrs::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html"]
    pub fn nodes(&self) -> CatNodes {
        CatNodes::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html"]
    pub fn pending_tasks(&self) -> CatPendingTasks {
        CatPendingTasks::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html"]
    pub fn plugins(&self) -> CatPlugins {
        CatPlugins::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html"]
    pub fn recovery(&self) -> CatRecovery {
        CatRecovery::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html"]
    pub fn repositories(&self) -> CatRepositories {
        CatRepositories::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html"]
    pub fn segments(&self) -> CatSegments {
        CatSegments::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html"]
    pub fn shards(&self) -> CatShards {
        CatShards::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html"]
    pub fn snapshots(&self) -> CatSnapshots {
        CatSnapshots::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn tasks(&self) -> CatTasks {
        CatTasks::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html"]
    pub fn templates(&self) -> CatTemplates {
        CatTemplates::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html"]
    pub fn thread_pool(&self) -> CatThreadPool {
        CatThreadPool::new(self.client.clone())
    }
}
impl Elasticsearch {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> Cat {
        Cat::new(self.clone())
    }
}