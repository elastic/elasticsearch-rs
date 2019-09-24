

use super::super::client::ElasticsearchClient;
use super::super::http_method::HttpMethod;
use crate::client::Sender;
use crate::response::ElasticsearchResponse;
use reqwest::header::HeaderMap;
use reqwest::{Error, Request, Response, Result, StatusCode};
use serde::de::DeserializeOwned;
#[derive(Default)]
pub struct CatAliasesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatAliasesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatAliasesBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatAliasesBuilder {
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
pub struct CatAllocationBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatAllocationBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatAllocationBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatAllocationBuilder {
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
pub struct CatCountBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatCountBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatCountBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatCountBuilder {
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
pub struct CatFielddataBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    fields: Option<Vec<String>>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatFielddataBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatFielddataBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
        self
    }
    #[doc = "A comma-separated list of fields to return in the output"]
    pub fn fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.fields = fields;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatFielddataBuilder {
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
pub struct CatHealthBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    ts: Option<bool>,
    v: Option<bool>,
}
impl CatHealthBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatHealthBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
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
impl Sender for CatHealthBuilder {
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
pub struct CatHelpBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    help: Option<bool>,
    s: Option<Vec<String>>,
}
impl CatHelpBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatHelpBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
}
impl Sender for CatHelpBuilder {
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
pub struct CatIndicesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    format: Option<String>,
    h: Option<Vec<String>>,
    health: Option<i32>,
    help: Option<bool>,
    include_unloaded_segments: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    pri: Option<bool>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatIndicesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatIndicesBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
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
    pub fn health(mut self, health: Option<i32>) -> Self {
        self.health = health;
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: Option<bool>) -> Self {
        self.help = help;
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
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatIndicesBuilder {
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
pub struct CatMasterBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatMasterBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatMasterBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatMasterBuilder {
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
pub struct CatNodeattrsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatNodeattrsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatNodeattrsBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatNodeattrsBuilder {
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
pub struct CatNodesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    full_id: Option<bool>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatNodesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatNodesBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatNodesBuilder {
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
pub struct CatPendingTasksBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatPendingTasksBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatPendingTasksBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatPendingTasksBuilder {
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
pub struct CatPluginsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatPluginsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatPluginsBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatPluginsBuilder {
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
pub struct CatRecoveryBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatRecoveryBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatRecoveryBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
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
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: Option<String>) -> Self {
        self.master_timeout = master_timeout;
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatRecoveryBuilder {
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
pub struct CatRepositoriesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatRepositoriesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatRepositoriesBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatRepositoriesBuilder {
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
pub struct CatSegmentsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatSegmentsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatSegmentsBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatSegmentsBuilder {
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
pub struct CatShardsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    bytes: Option<i32>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatShardsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatShardsBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Option<i32>) -> Self {
        self.bytes = bytes;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatShardsBuilder {
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
pub struct CatSnapshotsBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatSnapshotsBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatSnapshotsBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatSnapshotsBuilder {
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
pub struct CatTasksBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    actions: Option<Vec<String>>,
    detailed: Option<bool>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    node_id: Option<Vec<String>>,
    parent_task: Option<i64>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatTasksBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatTasksBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
        self
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatTasksBuilder {
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
pub struct CatTemplatesBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    v: Option<bool>,
}
impl CatTemplatesBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatTemplatesBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatTemplatesBuilder {
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
pub struct CatThreadPoolBuilder {
    client: ElasticsearchClient,
    error_trace: Option<bool>,
    filter_path: Option<Vec<String>>,
    human: Option<bool>,
    pretty: Option<bool>,
    source: Option<String>,
    format: Option<String>,
    h: Option<Vec<String>>,
    help: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<String>,
    s: Option<Vec<String>>,
    size: Option<i32>,
    v: Option<bool>,
}
impl CatThreadPoolBuilder {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatThreadPoolBuilder {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: Option<String>) -> Self {
        self.source = source;
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
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: Option<Vec<String>>) -> Self {
        self.s = s;
        self
    }
    #[doc = "The multiplier in which to display values"]
    pub fn size(mut self, size: Option<i32>) -> Self {
        self.size = size;
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: Option<bool>) -> Self {
        self.v = v;
        self
    }
}
impl Sender for CatThreadPoolBuilder {
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
pub struct CatClient {
    client: ElasticsearchClient,
}
impl CatClient {
    pub fn new(client: ElasticsearchClient) -> Self {
        CatClient { client }
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-alias.html"]
    pub fn aliases(&self) -> CatAliasesBuilder {
        CatAliasesBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-allocation.html"]
    pub fn allocation(&self) -> CatAllocationBuilder {
        CatAllocationBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-count.html"]
    pub fn count(&self) -> CatCountBuilder {
        CatCountBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-fielddata.html"]
    pub fn fielddata(&self) -> CatFielddataBuilder {
        CatFielddataBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-health.html"]
    pub fn health(&self) -> CatHealthBuilder {
        CatHealthBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat.html"]
    pub fn help(&self) -> CatHelpBuilder {
        CatHelpBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-indices.html"]
    pub fn indices(&self) -> CatIndicesBuilder {
        CatIndicesBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-master.html"]
    pub fn master(&self) -> CatMasterBuilder {
        CatMasterBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodeattrs.html"]
    pub fn nodeattrs(&self) -> CatNodeattrsBuilder {
        CatNodeattrsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-nodes.html"]
    pub fn nodes(&self) -> CatNodesBuilder {
        CatNodesBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-pending-tasks.html"]
    pub fn pending_tasks(&self) -> CatPendingTasksBuilder {
        CatPendingTasksBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-plugins.html"]
    pub fn plugins(&self) -> CatPluginsBuilder {
        CatPluginsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-recovery.html"]
    pub fn recovery(&self) -> CatRecoveryBuilder {
        CatRecoveryBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-repositories.html"]
    pub fn repositories(&self) -> CatRepositoriesBuilder {
        CatRepositoriesBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-segments.html"]
    pub fn segments(&self) -> CatSegmentsBuilder {
        CatSegmentsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html"]
    pub fn shards(&self) -> CatShardsBuilder {
        CatShardsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-snapshots.html"]
    pub fn snapshots(&self) -> CatSnapshotsBuilder {
        CatSnapshotsBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html"]
    pub fn tasks(&self) -> CatTasksBuilder {
        CatTasksBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-templates.html"]
    pub fn templates(&self) -> CatTemplatesBuilder {
        CatTemplatesBuilder::new(self.client.clone())
    }
    #[doc = "http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-thread-pool.html"]
    pub fn thread_pool(&self) -> CatThreadPoolBuilder {
        CatThreadPoolBuilder::new(self.client.clone())
    }
}
impl ElasticsearchClient {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> CatClient {
        CatClient::new(self.clone())
    }
}
