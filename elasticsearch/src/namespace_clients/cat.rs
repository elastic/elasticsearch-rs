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
use crate::{
    client::Elasticsearch,
    enums::*,
    error::ElasticsearchError,
    http_method::HttpMethod,
    request::{Body, JsonBody, NdBody},
    response::ElasticsearchResponse,
};
use reqwest::{header::HeaderMap, Error, Request, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_with;
use std::borrow::Cow;
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Aliases API"]
pub enum CatAliasesUrlParts<'a> {
    None,
    Name(&'a [&'a str]),
}
impl<'a> CatAliasesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatAliasesUrlParts::None => "/_cat/aliases".into(),
            CatAliasesUrlParts::Name(ref name) => {
                let name_str = name.join(",");
                let mut p = String::with_capacity(14usize + name_str.len());
                p.push_str("/_cat/aliases/");
                p.push_str(name_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Aliases API"]
pub struct CatAliases<'a> {
    client: Elasticsearch,
    parts: CatAliasesUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatAliases<'a> {
    pub fn new(client: Elasticsearch, parts: CatAliasesUrlParts<'a>) -> Self {
        CatAliases {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Aliases API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Allocation API"]
pub enum CatAllocationUrlParts<'a> {
    None,
    NodeId(&'a [&'a str]),
}
impl<'a> CatAllocationUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatAllocationUrlParts::None => "/_cat/allocation".into(),
            CatAllocationUrlParts::NodeId(ref node_id) => {
                let node_id_str = node_id.join(",");
                let mut p = String::with_capacity(17usize + node_id_str.len());
                p.push_str("/_cat/allocation/");
                p.push_str(node_id_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Allocation API"]
pub struct CatAllocation<'a> {
    client: Elasticsearch,
    parts: CatAllocationUrlParts<'a>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatAllocation<'a> {
    pub fn new(client: Elasticsearch, parts: CatAllocationUrlParts<'a>) -> Self {
        CatAllocation {
            client,
            parts,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Allocation API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Count API"]
pub enum CatCountUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> CatCountUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatCountUrlParts::None => "/_cat/count".into(),
            CatCountUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(12usize + index_str.len());
                p.push_str("/_cat/count/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Count API"]
pub struct CatCount<'a> {
    client: Elasticsearch,
    parts: CatCountUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatCount<'a> {
    pub fn new(client: Elasticsearch, parts: CatCountUrlParts<'a>) -> Self {
        CatCount {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Count API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Fielddata API"]
pub enum CatFielddataUrlParts<'a> {
    None,
    Fields(&'a [&'a str]),
}
impl<'a> CatFielddataUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatFielddataUrlParts::None => "/_cat/fielddata".into(),
            CatFielddataUrlParts::Fields(ref fields) => {
                let fields_str = fields.join(",");
                let mut p = String::with_capacity(16usize + fields_str.len());
                p.push_str("/_cat/fielddata/");
                p.push_str(fields_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Fielddata API"]
pub struct CatFielddata<'a> {
    client: Elasticsearch,
    parts: CatFielddataUrlParts<'a>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    fields: Option<&'a [&'a str]>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatFielddata<'a> {
    pub fn new(client: Elasticsearch, parts: CatFielddataUrlParts<'a>) -> Self {
        CatFielddata {
            client,
            parts,
            bytes: None,
            error_trace: None,
            fields: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of fields to return in the output"]
    pub fn fields(mut self, fields: &'a [&'a str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Fielddata API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(rename = "fields", serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'a [&'a str]>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                fields: self.fields,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Health API"]
pub enum CatHealthUrlParts {
    None,
}
impl CatHealthUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatHealthUrlParts::None => "/_cat/health".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Health API"]
pub struct CatHealth<'a> {
    client: Elasticsearch,
    parts: CatHealthUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    ts: Option<bool>,
    v: Option<bool>,
}
impl<'a> CatHealth<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatHealth {
            client,
            parts: CatHealthUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            ts: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Set to false to disable timestamping"]
    pub fn ts(mut self, ts: bool) -> Self {
        self.ts = Some(ts);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Health API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "ts")]
                ts: Option<bool>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                ts: self.ts,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Help API"]
pub enum CatHelpUrlParts {
    None,
}
impl CatHelpUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatHelpUrlParts::None => "/_cat".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Help API"]
pub struct CatHelp<'a> {
    client: Elasticsearch,
    parts: CatHelpUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
}
impl<'a> CatHelp<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatHelp {
            client,
            parts: CatHelpUrlParts::None,
            error_trace: None,
            filter_path: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Help API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Indices API"]
pub enum CatIndicesUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> CatIndicesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatIndicesUrlParts::None => "/_cat/indices".into(),
            CatIndicesUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(14usize + index_str.len());
                p.push_str("/_cat/indices/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Indices API"]
pub struct CatIndices<'a> {
    client: Elasticsearch,
    parts: CatIndicesUrlParts<'a>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    health: Option<Health>,
    help: Option<bool>,
    human: Option<bool>,
    include_unloaded_segments: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    pri: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatIndices<'a> {
    pub fn new(client: Elasticsearch, parts: CatIndicesUrlParts<'a>) -> Self {
        CatIndices {
            client,
            parts,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            health: None,
            help: None,
            human: None,
            include_unloaded_segments: None,
            local: None,
            master_timeout: None,
            pretty: None,
            pri: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "A health status (\"green\", \"yellow\", or \"red\" to filter only indices matching the specified health status"]
    pub fn health(mut self, health: Health) -> Self {
        self.health = Some(health);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "If set to true segment stats will include stats for segments that are not currently loaded into memory"]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: bool) -> Self {
        self.include_unloaded_segments = Some(include_unloaded_segments);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Set to true to return stats only for primary shards"]
    pub fn pri(mut self, pri: bool) -> Self {
        self.pri = Some(pri);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Indices API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "health")]
                health: Option<Health>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "include_unloaded_segments")]
                include_unloaded_segments: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "pri")]
                pri: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                health: self.health,
                help: self.help,
                human: self.human,
                include_unloaded_segments: self.include_unloaded_segments,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                pri: self.pri,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Master API"]
pub enum CatMasterUrlParts {
    None,
}
impl CatMasterUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatMasterUrlParts::None => "/_cat/master".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Master API"]
pub struct CatMaster<'a> {
    client: Elasticsearch,
    parts: CatMasterUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatMaster<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatMaster {
            client,
            parts: CatMasterUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Master API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Nodeattrs API"]
pub enum CatNodeattrsUrlParts {
    None,
}
impl CatNodeattrsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatNodeattrsUrlParts::None => "/_cat/nodeattrs".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Nodeattrs API"]
pub struct CatNodeattrs<'a> {
    client: Elasticsearch,
    parts: CatNodeattrsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatNodeattrs<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatNodeattrs {
            client,
            parts: CatNodeattrsUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Nodeattrs API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Nodes API"]
pub enum CatNodesUrlParts {
    None,
}
impl CatNodesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatNodesUrlParts::None => "/_cat/nodes".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Nodes API"]
pub struct CatNodes<'a> {
    client: Elasticsearch,
    parts: CatNodesUrlParts,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    full_id: Option<bool>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatNodes<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatNodes {
            client,
            parts: CatNodesUrlParts::None,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            full_id: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Return the full node ID instead of the shortened version (default: false)"]
    pub fn full_id(mut self, full_id: bool) -> Self {
        self.full_id = Some(full_id);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Nodes API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "full_id")]
                full_id: Option<bool>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                full_id: self.full_id,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Pending Tasks API"]
pub enum CatPendingTasksUrlParts {
    None,
}
impl CatPendingTasksUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatPendingTasksUrlParts::None => "/_cat/pending_tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Pending Tasks API"]
pub struct CatPendingTasks<'a> {
    client: Elasticsearch,
    parts: CatPendingTasksUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatPendingTasks<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatPendingTasks {
            client,
            parts: CatPendingTasksUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Pending Tasks API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Plugins API"]
pub enum CatPluginsUrlParts {
    None,
}
impl CatPluginsUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatPluginsUrlParts::None => "/_cat/plugins".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Plugins API"]
pub struct CatPlugins<'a> {
    client: Elasticsearch,
    parts: CatPluginsUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatPlugins<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatPlugins {
            client,
            parts: CatPluginsUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Plugins API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Recovery API"]
pub enum CatRecoveryUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> CatRecoveryUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatRecoveryUrlParts::None => "/_cat/recovery".into(),
            CatRecoveryUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/_cat/recovery/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Recovery API"]
pub struct CatRecovery<'a> {
    client: Elasticsearch,
    parts: CatRecoveryUrlParts<'a>,
    active_only: Option<bool>,
    bytes: Option<Bytes>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<&'a [&'a str]>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatRecovery<'a> {
    pub fn new(client: Elasticsearch, parts: CatRecoveryUrlParts<'a>) -> Self {
        CatRecovery {
            client,
            parts,
            active_only: None,
            bytes: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            index: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "If `true`, the response only includes ongoing shard recoveries"]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "If `true`, the response includes detailed information about shard recoveries"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Comma-separated list or wildcard expression of index names to limit the returned information"]
    pub fn index(mut self, index: &'a [&'a str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Recovery API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "active_only")]
                active_only: Option<bool>,
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "index", serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'a [&'a str]>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
                bytes: self.bytes,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                index: self.index,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Repositories API"]
pub enum CatRepositoriesUrlParts {
    None,
}
impl CatRepositoriesUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatRepositoriesUrlParts::None => "/_cat/repositories".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Repositories API"]
pub struct CatRepositories<'a> {
    client: Elasticsearch,
    parts: CatRepositoriesUrlParts,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatRepositories<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatRepositories {
            client,
            parts: CatRepositoriesUrlParts::None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Repositories API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Segments API"]
pub enum CatSegmentsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> CatSegmentsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatSegmentsUrlParts::None => "/_cat/segments".into(),
            CatSegmentsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(15usize + index_str.len());
                p.push_str("/_cat/segments/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Segments API"]
pub struct CatSegments<'a> {
    client: Elasticsearch,
    parts: CatSegmentsUrlParts<'a>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatSegments<'a> {
    pub fn new(client: Elasticsearch, parts: CatSegmentsUrlParts<'a>) -> Self {
        CatSegments {
            client,
            parts,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Segments API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Shards API"]
pub enum CatShardsUrlParts<'a> {
    None,
    Index(&'a [&'a str]),
}
impl<'a> CatShardsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatShardsUrlParts::None => "/_cat/shards".into(),
            CatShardsUrlParts::Index(ref index) => {
                let index_str = index.join(",");
                let mut p = String::with_capacity(13usize + index_str.len());
                p.push_str("/_cat/shards/");
                p.push_str(index_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Shards API"]
pub struct CatShards<'a> {
    client: Elasticsearch,
    parts: CatShardsUrlParts<'a>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatShards<'a> {
    pub fn new(client: Elasticsearch, parts: CatShardsUrlParts<'a>) -> Self {
        CatShards {
            client,
            parts,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The unit in which to display byte values"]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Shards API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "bytes")]
                bytes: Option<Bytes>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Snapshots API"]
pub enum CatSnapshotsUrlParts<'a> {
    None,
    Repository(&'a [&'a str]),
}
impl<'a> CatSnapshotsUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatSnapshotsUrlParts::None => "/_cat/snapshots".into(),
            CatSnapshotsUrlParts::Repository(ref repository) => {
                let repository_str = repository.join(",");
                let mut p = String::with_capacity(16usize + repository_str.len());
                p.push_str("/_cat/snapshots/");
                p.push_str(repository_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Snapshots API"]
pub struct CatSnapshots<'a> {
    client: Elasticsearch,
    parts: CatSnapshotsUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatSnapshots<'a> {
    pub fn new(client: Elasticsearch, parts: CatSnapshotsUrlParts<'a>) -> Self {
        CatSnapshots {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Set to true to ignore unavailable snapshots"]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Snapshots API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "ignore_unavailable")]
                ignore_unavailable: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Tasks API"]
pub enum CatTasksUrlParts {
    None,
}
impl CatTasksUrlParts {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatTasksUrlParts::None => "/_cat/tasks".into(),
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Tasks API"]
pub struct CatTasks<'a> {
    client: Elasticsearch,
    parts: CatTasksUrlParts,
    actions: Option<&'a [&'a str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    node_id: Option<&'a [&'a str]>,
    parent_task: Option<i64>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a> CatTasks<'a> {
    pub fn new(client: Elasticsearch) -> Self {
        CatTasks {
            client,
            parts: CatTasksUrlParts::None,
            actions: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            node_id: None,
            parent_task: None,
            pretty: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be returned. Leave empty to return all."]
    pub fn actions(mut self, actions: &'a [&'a str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "Return detailed task information (default: false)"]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn node_id(mut self, node_id: &'a [&'a str]) -> Self {
        self.node_id = Some(node_id);
        self
    }
    #[doc = "Return tasks with specified parent task id. Set to -1 to return all."]
    pub fn parent_task(mut self, parent_task: i64) -> Self {
        self.parent_task = Some(parent_task);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit in which to display time values"]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Tasks API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(
                    rename = "actions",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                actions: Option<&'a [&'a str]>,
                #[serde(rename = "detailed")]
                detailed: Option<bool>,
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(
                    rename = "node_id",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                node_id: Option<&'a [&'a str]>,
                #[serde(rename = "parent_task")]
                parent_task: Option<i64>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "time")]
                time: Option<Time>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                node_id: self.node_id,
                parent_task: self.parent_task,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Templates API"]
pub enum CatTemplatesUrlParts<'a> {
    None,
    Name(&'a str),
}
impl<'a> CatTemplatesUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatTemplatesUrlParts::None => "/_cat/templates".into(),
            CatTemplatesUrlParts::Name(ref name) => {
                let mut p = String::with_capacity(16usize + name.len());
                p.push_str("/_cat/templates/");
                p.push_str(name.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Templates API"]
pub struct CatTemplates<'a> {
    client: Elasticsearch,
    parts: CatTemplatesUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatTemplates<'a> {
    pub fn new(client: Elasticsearch, parts: CatTemplatesUrlParts<'a>) -> Self {
        CatTemplates {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Templates API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "Url parts for the Cat Thread Pool API"]
pub enum CatThreadPoolUrlParts<'a> {
    None,
    ThreadPoolPatterns(&'a [&'a str]),
}
impl<'a> CatThreadPoolUrlParts<'a> {
    pub fn build(self) -> Cow<'static, str> {
        match self {
            CatThreadPoolUrlParts::None => "/_cat/thread_pool".into(),
            CatThreadPoolUrlParts::ThreadPoolPatterns(ref thread_pool_patterns) => {
                let thread_pool_patterns_str = thread_pool_patterns.join(",");
                let mut p = String::with_capacity(18usize + thread_pool_patterns_str.len());
                p.push_str("/_cat/thread_pool/");
                p.push_str(thread_pool_patterns_str.as_ref());
                p.into()
            }
        }
    }
}
#[derive(Clone, Debug)]
#[doc = "Request builder for the Cat Thread Pool API"]
pub struct CatThreadPool<'a> {
    client: Elasticsearch,
    parts: CatThreadPoolUrlParts<'a>,
    error_trace: Option<bool>,
    filter_path: Option<&'a [&'a str]>,
    format: Option<&'a str>,
    h: Option<&'a [&'a str]>,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'a str>,
    pretty: Option<bool>,
    s: Option<&'a [&'a str]>,
    size: Option<Size>,
    source: Option<&'a str>,
    v: Option<bool>,
}
impl<'a> CatThreadPool<'a> {
    pub fn new(client: Elasticsearch, parts: CatThreadPoolUrlParts<'a>) -> Self {
        CatThreadPool {
            client,
            parts,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            s: None,
            size: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'a [&'a str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "a short version of the Accept header, e.g. json, yaml"]
    pub fn format(mut self, format: &'a str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Comma-separated list of column names to display"]
    pub fn h(mut self, h: &'a [&'a str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Return help information"]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Return local information, do not retrieve the state from master node (default: false)"]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Explicit operation timeout for connection to master node"]
    pub fn master_timeout(mut self, master_timeout: &'a str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Comma-separated list of column names or column aliases to sort by"]
    pub fn s(mut self, s: &'a [&'a str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The multiplier in which to display values"]
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Verbose mode. Display column headers"]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous request to the Cat Thread Pool API that can be awaited"]
    pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
        let path = self.parts.build();
        let method = HttpMethod::Get;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'a> {
                #[serde(rename = "error_trace")]
                error_trace: Option<bool>,
                #[serde(
                    rename = "filter_path",
                    serialize_with = "crate::client::serialize_coll_qs"
                )]
                filter_path: Option<&'a [&'a str]>,
                #[serde(rename = "format")]
                format: Option<&'a str>,
                #[serde(rename = "h", serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'a [&'a str]>,
                #[serde(rename = "help")]
                help: Option<bool>,
                #[serde(rename = "human")]
                human: Option<bool>,
                #[serde(rename = "local")]
                local: Option<bool>,
                #[serde(rename = "master_timeout")]
                master_timeout: Option<&'a str>,
                #[serde(rename = "pretty")]
                pretty: Option<bool>,
                #[serde(rename = "s", serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'a [&'a str]>,
                #[serde(rename = "size")]
                size: Option<Size>,
                #[serde(rename = "source")]
                source: Option<&'a str>,
                #[serde(rename = "v")]
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                size: self.size,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .client
            .send(method, &path, query_string.as_ref(), body)
            .await?;
        Ok(response)
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
    #[doc = "Shows information about currently configured aliases to indices including filter and routing infos."]
    pub fn aliases<'a>(&self, parts: CatAliasesUrlParts<'a>) -> CatAliases<'a> {
        CatAliases::new(self.client.clone(), parts)
    }
    #[doc = "Provides a snapshot of how many shards are allocated to each data node and how much disk space they are using."]
    pub fn allocation<'a>(&self, parts: CatAllocationUrlParts<'a>) -> CatAllocation<'a> {
        CatAllocation::new(self.client.clone(), parts)
    }
    #[doc = "Provides quick access to the document count of the entire cluster, or individual indices."]
    pub fn count<'a>(&self, parts: CatCountUrlParts<'a>) -> CatCount<'a> {
        CatCount::new(self.client.clone(), parts)
    }
    #[doc = "Shows how much heap memory is currently being used by fielddata on every data node in the cluster."]
    pub fn fielddata<'a>(&self, parts: CatFielddataUrlParts<'a>) -> CatFielddata<'a> {
        CatFielddata::new(self.client.clone(), parts)
    }
    #[doc = "Returns a concise representation of the cluster health."]
    pub fn health<'a>(&self) -> CatHealth<'a> {
        CatHealth::new(self.client.clone())
    }
    #[doc = "Returns help for the Cat APIs."]
    pub fn help<'a>(&self) -> CatHelp<'a> {
        CatHelp::new(self.client.clone())
    }
    #[doc = "Returns information about indices: number of primaries and replicas, document counts, disk size, ..."]
    pub fn indices<'a>(&self, parts: CatIndicesUrlParts<'a>) -> CatIndices<'a> {
        CatIndices::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about the master node."]
    pub fn master<'a>(&self) -> CatMaster<'a> {
        CatMaster::new(self.client.clone())
    }
    #[doc = "Returns information about custom node attributes."]
    pub fn nodeattrs<'a>(&self) -> CatNodeattrs<'a> {
        CatNodeattrs::new(self.client.clone())
    }
    #[doc = "Returns basic statistics about performance of cluster nodes."]
    pub fn nodes<'a>(&self) -> CatNodes<'a> {
        CatNodes::new(self.client.clone())
    }
    #[doc = "Returns a concise representation of the cluster pending tasks."]
    pub fn pending_tasks<'a>(&self) -> CatPendingTasks<'a> {
        CatPendingTasks::new(self.client.clone())
    }
    #[doc = "Returns information about installed plugins across nodes node."]
    pub fn plugins<'a>(&self) -> CatPlugins<'a> {
        CatPlugins::new(self.client.clone())
    }
    #[doc = "Returns information about index shard recoveries, both on-going completed."]
    pub fn recovery<'a>(&self, parts: CatRecoveryUrlParts<'a>) -> CatRecovery<'a> {
        CatRecovery::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about snapshot repositories registered in the cluster."]
    pub fn repositories<'a>(&self) -> CatRepositories<'a> {
        CatRepositories::new(self.client.clone())
    }
    #[doc = "Provides low-level information about the segments in the shards of an index."]
    pub fn segments<'a>(&self, parts: CatSegmentsUrlParts<'a>) -> CatSegments<'a> {
        CatSegments::new(self.client.clone(), parts)
    }
    #[doc = "Provides a detailed view of shard allocation on nodes."]
    pub fn shards<'a>(&self, parts: CatShardsUrlParts<'a>) -> CatShards<'a> {
        CatShards::new(self.client.clone(), parts)
    }
    #[doc = "Returns all snapshots in a specific repository."]
    pub fn snapshots<'a>(&self, parts: CatSnapshotsUrlParts<'a>) -> CatSnapshots<'a> {
        CatSnapshots::new(self.client.clone(), parts)
    }
    #[doc = "Returns information about the tasks currently executing on one or more nodes in the cluster."]
    pub fn tasks<'a>(&self) -> CatTasks<'a> {
        CatTasks::new(self.client.clone())
    }
    #[doc = "Returns information about existing templates."]
    pub fn templates<'a>(&self, parts: CatTemplatesUrlParts<'a>) -> CatTemplates<'a> {
        CatTemplates::new(self.client.clone(), parts)
    }
    #[doc = "Returns cluster-wide thread pool statistics per node.\nBy default the active, queue and rejected statistics are returned for all thread pools."]
    pub fn thread_pool<'a>(&self, parts: CatThreadPoolUrlParts<'a>) -> CatThreadPool<'a> {
        CatThreadPool::new(self.client.clone(), parts)
    }
}
impl Elasticsearch {
    #[doc = "Cat APIs"]
    pub fn cat(&self) -> Cat {
        Cat::new(self.clone())
    }
}
