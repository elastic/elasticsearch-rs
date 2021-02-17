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

#![cfg(feature = "experimental-apis")]
#![doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#![allow(unused_imports)]
use crate::{
    client::Elasticsearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Text Structure Find Structure API"]
pub enum TextStructureFindStructureParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl TextStructureFindStructureParts {
    #[doc = "Builds a relative URL path to the Text Structure Find Structure API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TextStructureFindStructureParts::None => "/_text_structure/find_structure".into(),
        }
    }
}
#[doc = "Builder for the [Text Structure Find Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/find-structure.html)\n\nFinds the structure of a text file. The text file must contain data that is suitable to be ingested into Elasticsearch."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct TextStructureFindStructure<'a, 'b, B> {
    transport: &'a Transport,
    parts: TextStructureFindStructureParts,
    body: Option<B>,
    charset: Option<&'b str>,
    column_names: Option<&'b [&'b str]>,
    delimiter: Option<&'b str>,
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
#[cfg(feature = "experimental-apis")]
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
            body: Some(NdBody(body)),
            charset: self.charset,
            column_names: self.column_names,
            delimiter: self.delimiter,
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
        let method = Method::Post;
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
#[doc = "Namespace client for TextStructure APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct TextStructure<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> TextStructure<'a> {
    #[doc = "Creates a new instance of [TextStructure]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Text Structure Find Structure API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/find-structure.html)\n\nFinds the structure of a text file. The text file must contain data that is suitable to be ingested into Elasticsearch."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn find_structure<'b>(&'a self) -> TextStructureFindStructure<'a, 'b, ()> {
        TextStructureFindStructure::new(self.transport())
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for TextStructure APIs"]
    pub fn text_structure(&self) -> TextStructure {
        TextStructure::new(self.transport())
    }
}
