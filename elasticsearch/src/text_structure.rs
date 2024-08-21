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

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! Text structure APIs
//!
//! Determines the structure of text and other information that will be useful to import its contents to an Elasticsearch
//! index.

#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        self,
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Text Structure Find Field Structure API"]
pub enum TextStructureFindFieldStructureParts {
    #[doc = "No parts"]
    None,
}
impl TextStructureFindFieldStructureParts {
    #[doc = "Builds a relative URL path to the Text Structure Find Field Structure API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TextStructureFindFieldStructureParts::None => {
                "/_text_structure/find_field_structure".into()
            }
        }
    }
}
#[doc = "Builder for the [Text Structure Find Field Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-field-structure.html)\n\nFinds the structure of a text field in an index."]
#[derive(Clone, Debug)]
pub struct TextStructureFindFieldStructure<'a, 'b> {
    transport: &'a Transport,
    parts: TextStructureFindFieldStructureParts,
    column_names: Option<&'b [&'b str]>,
    delimiter: Option<&'b str>,
    documents_to_sample: Option<i32>,
    ecs_compatibility: Option<&'b str>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    field: Option<&'b str>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<Format>,
    grok_pattern: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    quote: Option<&'b str>,
    request_timeout: Option<Duration>,
    should_trim_fields: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    timestamp_field: Option<&'b str>,
    timestamp_format: Option<&'b str>,
}
impl<'a, 'b> TextStructureFindFieldStructure<'a, 'b> {
    #[doc = "Creates a new instance of [TextStructureFindFieldStructure]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TextStructureFindFieldStructure {
            transport,
            parts: TextStructureFindFieldStructureParts::None,
            headers,
            column_names: None,
            delimiter: None,
            documents_to_sample: None,
            ecs_compatibility: None,
            error_trace: None,
            explain: None,
            field: None,
            filter_path: None,
            format: None,
            grok_pattern: None,
            human: None,
            index: None,
            pretty: None,
            quote: None,
            request_timeout: None,
            should_trim_fields: None,
            source: None,
            timeout: None,
            timestamp_field: None,
            timestamp_format: None,
        }
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: &'b [&'b str]) -> Self {
        self.column_names = Some(column_names);
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "How many documents should be included in the analysis"]
    pub fn documents_to_sample(mut self, documents_to_sample: i32) -> Self {
        self.documents_to_sample = Some(documents_to_sample);
        self
    }
    #[doc = "Optional parameter to specify the compatibility mode with ECS Grok patterns - may be either 'v1' or 'disabled'"]
    pub fn ecs_compatibility(mut self, ecs_compatibility: &'b str) -> Self {
        self.ecs_compatibility = Some(ecs_compatibility);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "The field that should be analyzed"]
    pub fn field(mut self, field: &'b str) -> Self {
        self.field = Some(field);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: &'b str) -> Self {
        self.grok_pattern = Some(grok_pattern);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "The index containing the analyzed field"]
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Optional parameter to specify the quote character for a delimited file - must be a single character"]
    pub fn quote(mut self, quote: &'b str) -> Self {
        self.quote = Some(quote);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: bool) -> Self {
        self.should_trim_fields = Some(should_trim_fields);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: &'b str) -> Self {
        self.timestamp_field = Some(timestamp_field);
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: &'b str) -> Self {
        self.timestamp_format = Some(timestamp_format);
        self
    }
    #[doc = "Creates an asynchronous call to the Text Structure Find Field Structure API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                column_names: Option<&'b [&'b str]>,
                delimiter: Option<&'b str>,
                documents_to_sample: Option<i32>,
                ecs_compatibility: Option<&'b str>,
                error_trace: Option<bool>,
                explain: Option<bool>,
                field: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<Format>,
                grok_pattern: Option<&'b str>,
                human: Option<bool>,
                index: Option<&'b str>,
                pretty: Option<bool>,
                quote: Option<&'b str>,
                should_trim_fields: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                timestamp_field: Option<&'b str>,
                timestamp_format: Option<&'b str>,
            }
            let query_params = QueryParams {
                column_names: self.column_names,
                delimiter: self.delimiter,
                documents_to_sample: self.documents_to_sample,
                ecs_compatibility: self.ecs_compatibility,
                error_trace: self.error_trace,
                explain: self.explain,
                field: self.field,
                filter_path: self.filter_path,
                format: self.format,
                grok_pattern: self.grok_pattern,
                human: self.human,
                index: self.index,
                pretty: self.pretty,
                quote: self.quote,
                should_trim_fields: self.should_trim_fields,
                source: self.source,
                timeout: self.timeout,
                timestamp_field: self.timestamp_field,
                timestamp_format: self.timestamp_format,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Text Structure Find Message Structure API"]
pub enum TextStructureFindMessageStructureParts {
    #[doc = "No parts"]
    None,
}
impl TextStructureFindMessageStructureParts {
    #[doc = "Builds a relative URL path to the Text Structure Find Message Structure API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TextStructureFindMessageStructureParts::None => {
                "/_text_structure/find_message_structure".into()
            }
        }
    }
}
#[doc = "Builder for the [Text Structure Find Message Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-message-structure.html)\n\nFinds the structure of a list of messages. The messages must contain data that is suitable to be ingested into Elasticsearch."]
#[derive(Clone, Debug)]
pub struct TextStructureFindMessageStructure<'a, 'b, B> {
    transport: &'a Transport,
    parts: TextStructureFindMessageStructureParts,
    body: Option<B>,
    column_names: Option<&'b [&'b str]>,
    delimiter: Option<&'b str>,
    ecs_compatibility: Option<&'b str>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<Format>,
    grok_pattern: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    quote: Option<&'b str>,
    request_timeout: Option<Duration>,
    should_trim_fields: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    timestamp_field: Option<&'b str>,
    timestamp_format: Option<&'b str>,
}
impl<'a, 'b, B> TextStructureFindMessageStructure<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TextStructureFindMessageStructure]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TextStructureFindMessageStructure {
            transport,
            parts: TextStructureFindMessageStructureParts::None,
            headers,
            body: None,
            column_names: None,
            delimiter: None,
            ecs_compatibility: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            format: None,
            grok_pattern: None,
            human: None,
            pretty: None,
            quote: None,
            request_timeout: None,
            should_trim_fields: None,
            source: None,
            timeout: None,
            timestamp_field: None,
            timestamp_format: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TextStructureFindMessageStructure<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TextStructureFindMessageStructure {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            column_names: self.column_names,
            delimiter: self.delimiter,
            ecs_compatibility: self.ecs_compatibility,
            error_trace: self.error_trace,
            explain: self.explain,
            filter_path: self.filter_path,
            format: self.format,
            grok_pattern: self.grok_pattern,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            quote: self.quote,
            request_timeout: self.request_timeout,
            should_trim_fields: self.should_trim_fields,
            source: self.source,
            timeout: self.timeout,
            timestamp_field: self.timestamp_field,
            timestamp_format: self.timestamp_format,
        }
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: &'b [&'b str]) -> Self {
        self.column_names = Some(column_names);
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Optional parameter to specify the compatibility mode with ECS Grok patterns - may be either 'v1' or 'disabled'"]
    pub fn ecs_compatibility(mut self, ecs_compatibility: &'b str) -> Self {
        self.ecs_compatibility = Some(ecs_compatibility);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: &'b str) -> Self {
        self.grok_pattern = Some(grok_pattern);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Optional parameter to specify the quote character for a delimited file - must be a single character"]
    pub fn quote(mut self, quote: &'b str) -> Self {
        self.quote = Some(quote);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: bool) -> Self {
        self.should_trim_fields = Some(should_trim_fields);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: &'b str) -> Self {
        self.timestamp_field = Some(timestamp_field);
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: &'b str) -> Self {
        self.timestamp_format = Some(timestamp_format);
        self
    }
    #[doc = "Creates an asynchronous call to the Text Structure Find Message Structure API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                column_names: Option<&'b [&'b str]>,
                delimiter: Option<&'b str>,
                ecs_compatibility: Option<&'b str>,
                error_trace: Option<bool>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<Format>,
                grok_pattern: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                quote: Option<&'b str>,
                should_trim_fields: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                timestamp_field: Option<&'b str>,
                timestamp_format: Option<&'b str>,
            }
            let query_params = QueryParams {
                column_names: self.column_names,
                delimiter: self.delimiter,
                ecs_compatibility: self.ecs_compatibility,
                error_trace: self.error_trace,
                explain: self.explain,
                filter_path: self.filter_path,
                format: self.format,
                grok_pattern: self.grok_pattern,
                human: self.human,
                pretty: self.pretty,
                quote: self.quote,
                should_trim_fields: self.should_trim_fields,
                source: self.source,
                timeout: self.timeout,
                timestamp_field: self.timestamp_field,
                timestamp_format: self.timestamp_format,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Text Structure Find Structure API"]
pub enum TextStructureFindStructureParts {
    #[doc = "No parts"]
    None,
}
impl TextStructureFindStructureParts {
    #[doc = "Builds a relative URL path to the Text Structure Find Structure API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TextStructureFindStructureParts::None => "/_text_structure/find_structure".into(),
        }
    }
}
#[doc = "Builder for the [Text Structure Find Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-structure.html)\n\nFinds the structure of a text file. The text file must contain data that is suitable to be ingested into Elasticsearch."]
#[derive(Clone, Debug)]
pub struct TextStructureFindStructure<'a, 'b, B> {
    transport: &'a Transport,
    parts: TextStructureFindStructureParts,
    body: Option<B>,
    charset: Option<&'b str>,
    column_names: Option<&'b [&'b str]>,
    delimiter: Option<&'b str>,
    ecs_compatibility: Option<&'b str>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<Format>,
    grok_pattern: Option<&'b str>,
    has_header_row: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    line_merge_size_limit: Option<i32>,
    lines_to_sample: Option<i32>,
    pretty: Option<bool>,
    quote: Option<&'b str>,
    request_timeout: Option<Duration>,
    should_trim_fields: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    timestamp_field: Option<&'b str>,
    timestamp_format: Option<&'b str>,
}
impl<'a, 'b, B> TextStructureFindStructure<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TextStructureFindStructure]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TextStructureFindStructure {
            transport,
            parts: TextStructureFindStructureParts::None,
            headers,
            body: None,
            charset: None,
            column_names: None,
            delimiter: None,
            ecs_compatibility: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            format: None,
            grok_pattern: None,
            has_header_row: None,
            human: None,
            line_merge_size_limit: None,
            lines_to_sample: None,
            pretty: None,
            quote: None,
            request_timeout: None,
            should_trim_fields: None,
            source: None,
            timeout: None,
            timestamp_field: None,
            timestamp_format: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> TextStructureFindStructure<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        TextStructureFindStructure {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody::new(body)),
            charset: self.charset,
            column_names: self.column_names,
            delimiter: self.delimiter,
            ecs_compatibility: self.ecs_compatibility,
            error_trace: self.error_trace,
            explain: self.explain,
            filter_path: self.filter_path,
            format: self.format,
            grok_pattern: self.grok_pattern,
            has_header_row: self.has_header_row,
            headers: self.headers,
            human: self.human,
            line_merge_size_limit: self.line_merge_size_limit,
            lines_to_sample: self.lines_to_sample,
            pretty: self.pretty,
            quote: self.quote,
            request_timeout: self.request_timeout,
            should_trim_fields: self.should_trim_fields,
            source: self.source,
            timeout: self.timeout,
            timestamp_field: self.timestamp_field,
            timestamp_format: self.timestamp_format,
        }
    }
    #[doc = "Optional parameter to specify the character set of the file"]
    pub fn charset(mut self, charset: &'b str) -> Self {
        self.charset = Some(charset);
        self
    }
    #[doc = "Optional parameter containing a comma separated list of the column names for a delimited file"]
    pub fn column_names(mut self, column_names: &'b [&'b str]) -> Self {
        self.column_names = Some(column_names);
        self
    }
    #[doc = "Optional parameter to specify the delimiter character for a delimited file - must be a single character"]
    pub fn delimiter(mut self, delimiter: &'b str) -> Self {
        self.delimiter = Some(delimiter);
        self
    }
    #[doc = "Optional parameter to specify the compatibility mode with ECS Grok patterns - may be either 'v1' or 'disabled'"]
    pub fn ecs_compatibility(mut self, ecs_compatibility: &'b str) -> Self {
        self.ecs_compatibility = Some(ecs_compatibility);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to include a commentary on how the structure was derived"]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Optional parameter to specify the high level file format"]
    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "Optional parameter to specify the Grok pattern that should be used to extract fields from messages in a semi-structured text file"]
    pub fn grok_pattern(mut self, grok_pattern: &'b str) -> Self {
        self.grok_pattern = Some(grok_pattern);
        self
    }
    #[doc = "Optional parameter to specify whether a delimited file includes the column names in its first row"]
    pub fn has_header_row(mut self, has_header_row: bool) -> Self {
        self.has_header_row = Some(has_header_row);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Return human readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Maximum number of characters permitted in a single message when lines are merged to create messages."]
    pub fn line_merge_size_limit(mut self, line_merge_size_limit: i32) -> Self {
        self.line_merge_size_limit = Some(line_merge_size_limit);
        self
    }
    #[doc = "How many lines of the file should be included in the analysis"]
    pub fn lines_to_sample(mut self, lines_to_sample: i32) -> Self {
        self.lines_to_sample = Some(lines_to_sample);
        self
    }
    #[doc = "Pretty format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Optional parameter to specify the quote character for a delimited file - must be a single character"]
    pub fn quote(mut self, quote: &'b str) -> Self {
        self.quote = Some(quote);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify whether the values between delimiters in a delimited file should have whitespace trimmed from them"]
    pub fn should_trim_fields(mut self, should_trim_fields: bool) -> Self {
        self.should_trim_fields = Some(should_trim_fields);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout after which the analysis will be aborted"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Optional parameter to specify the timestamp field in the file"]
    pub fn timestamp_field(mut self, timestamp_field: &'b str) -> Self {
        self.timestamp_field = Some(timestamp_field);
        self
    }
    #[doc = "Optional parameter to specify the timestamp format in the file - may be either a Joda or Java time format"]
    pub fn timestamp_format(mut self, timestamp_format: &'b str) -> Self {
        self.timestamp_format = Some(timestamp_format);
        self
    }
    #[doc = "Creates an asynchronous call to the Text Structure Find Structure API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = http::Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                charset: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                column_names: Option<&'b [&'b str]>,
                delimiter: Option<&'b str>,
                ecs_compatibility: Option<&'b str>,
                error_trace: Option<bool>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<Format>,
                grok_pattern: Option<&'b str>,
                has_header_row: Option<bool>,
                human: Option<bool>,
                line_merge_size_limit: Option<i32>,
                lines_to_sample: Option<i32>,
                pretty: Option<bool>,
                quote: Option<&'b str>,
                should_trim_fields: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                timestamp_field: Option<&'b str>,
                timestamp_format: Option<&'b str>,
            }
            let query_params = QueryParams {
                charset: self.charset,
                column_names: self.column_names,
                delimiter: self.delimiter,
                ecs_compatibility: self.ecs_compatibility,
                error_trace: self.error_trace,
                explain: self.explain,
                filter_path: self.filter_path,
                format: self.format,
                grok_pattern: self.grok_pattern,
                has_header_row: self.has_header_row,
                human: self.human,
                line_merge_size_limit: self.line_merge_size_limit,
                lines_to_sample: self.lines_to_sample,
                pretty: self.pretty,
                quote: self.quote,
                should_trim_fields: self.should_trim_fields,
                source: self.source,
                timeout: self.timeout,
                timestamp_field: self.timestamp_field,
                timestamp_format: self.timestamp_format,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Text Structure Test Grok Pattern API"]
pub enum TextStructureTestGrokPatternParts {
    #[doc = "No parts"]
    None,
}
impl TextStructureTestGrokPatternParts {
    #[doc = "Builds a relative URL path to the Text Structure Test Grok Pattern API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TextStructureTestGrokPatternParts::None => "/_text_structure/test_grok_pattern".into(),
        }
    }
}
#[doc = "Builder for the [Text Structure Test Grok Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/test-grok-pattern.html)\n\nTests a Grok pattern on some text."]
#[derive(Clone, Debug)]
pub struct TextStructureTestGrokPattern<'a, 'b, B> {
    transport: &'a Transport,
    parts: TextStructureTestGrokPatternParts,
    body: Option<B>,
    ecs_compatibility: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TextStructureTestGrokPattern<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TextStructureTestGrokPattern]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TextStructureTestGrokPattern {
            transport,
            parts: TextStructureTestGrokPatternParts::None,
            headers,
            body: None,
            ecs_compatibility: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TextStructureTestGrokPattern<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TextStructureTestGrokPattern {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            ecs_compatibility: self.ecs_compatibility,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
        }
    }
    #[doc = "Optional parameter to specify the compatibility mode with ECS Grok patterns - may be either 'v1' or 'disabled'"]
    pub fn ecs_compatibility(mut self, ecs_compatibility: &'b str) -> Self {
        self.ecs_compatibility = Some(ecs_compatibility);
        self
    }
    #[doc = "Include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to reduce the response."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
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
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Text Structure Test Grok Pattern API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => http::Method::Post,
            None => http::Method::Get,
        };
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                ecs_compatibility: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                ecs_compatibility: self.ecs_compatibility,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for TextStructure APIs"]
pub struct TextStructure<'a> {
    transport: &'a Transport,
}
impl<'a> TextStructure<'a> {
    #[doc = "Creates a new instance of [TextStructure]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Text Structure Find Field Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-field-structure.html)\n\nFinds the structure of a text field in an index."]
    pub fn find_field_structure<'b>(&'a self) -> TextStructureFindFieldStructure<'a, 'b> {
        TextStructureFindFieldStructure::new(self.transport())
    }
    #[doc = "[Text Structure Find Message Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-message-structure.html)\n\nFinds the structure of a list of messages. The messages must contain data that is suitable to be ingested into Elasticsearch."]
    pub fn find_message_structure<'b>(&'a self) -> TextStructureFindMessageStructure<'a, 'b, ()> {
        TextStructureFindMessageStructure::new(self.transport())
    }
    #[doc = "[Text Structure Find Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/find-structure.html)\n\nFinds the structure of a text file. The text file must contain data that is suitable to be ingested into Elasticsearch."]
    pub fn find_structure<'b>(&'a self) -> TextStructureFindStructure<'a, 'b, ()> {
        TextStructureFindStructure::new(self.transport())
    }
    #[doc = "[Text Structure Test Grok Pattern API](https://www.elastic.co/guide/en/elasticsearch/reference/8.15/test-grok-pattern.html)\n\nTests a Grok pattern on some text."]
    pub fn test_grok_pattern<'b>(&'a self) -> TextStructureTestGrokPattern<'a, 'b, ()> {
        TextStructureTestGrokPattern::new(self.transport())
    }
}
impl Elasticsearch {
    #[doc = "Creates a namespace client for TextStructure APIs"]
    pub fn text_structure(&self) -> TextStructure {
        TextStructure::new(self.transport())
    }
}
