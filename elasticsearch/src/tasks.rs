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

//! Task Management APIs
//!
//! [Manage tasks currently executing on one or more nodes in the cluster](https://www.elastic.co/guide/en/elasticsearch/reference/master/tasks.html).

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
#[doc = "API parts for the Tasks Cancel API"]
pub enum TasksCancelParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "TaskId"]
    TaskId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> TasksCancelParts<'b> {
    #[doc = "Builds a relative URL path to the Tasks Cancel API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksCancelParts::None => "/_tasks/_cancel".into(),
            TasksCancelParts::TaskId(ref task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_task_id.len());
                p.push_str("/_tasks/");
                p.push_str(encoded_task_id.as_ref());
                p.push_str("/_cancel");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Tasks Cancel API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nCancels a task, if it can be cancelled through an API."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct TasksCancel<'a, 'b, B> {
    transport: &'a Transport,
    parts: TasksCancelParts<'b>,
    actions: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    nodes: Option<&'b [&'b str]>,
    parent_task_id: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b, B> TasksCancel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TasksCancel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TasksCancelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TasksCancel {
            transport,
            parts,
            headers,
            actions: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be cancelled. Leave empty to cancel all."]
    pub fn actions(mut self, actions: &'b [&'b str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TasksCancel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TasksCancel {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            actions: self.actions,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            nodes: self.nodes,
            parent_task_id: self.parent_task_id,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_completion: self.wait_for_completion,
        }
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: &'b [&'b str]) -> Self {
        self.nodes = Some(nodes);
        self
    }
    #[doc = "Cancel tasks with specified parent task id (node_id:task_number). Set to -1 to cancel all."]
    pub fn parent_task_id(mut self, parent_task_id: &'b str) -> Self {
        self.parent_task_id = Some(parent_task_id);
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
    #[doc = "Should the request block until the cancellation of the task and its descendant tasks is completed. Defaults to false"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Tasks Cancel API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                actions: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                nodes: Option<&'b [&'b str]>,
                parent_task_id: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion: self.wait_for_completion,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Tasks Get API"]
pub enum TasksGetParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
#[cfg(feature = "experimental-apis")]
impl<'b> TasksGetParts<'b> {
    #[doc = "Builds a relative URL path to the Tasks Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksGetParts::TaskId(ref task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_task_id.len());
                p.push_str("/_tasks/");
                p.push_str(encoded_task_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Tasks Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nReturns information about a task."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct TasksGet<'a, 'b> {
    transport: &'a Transport,
    parts: TasksGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> TasksGet<'a, 'b> {
    #[doc = "Creates a new instance of [TasksGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TasksGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TasksGet {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
        }
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Tasks Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
#[cfg(feature = "experimental-apis")]
#[derive(Debug, Clone, PartialEq)]
#[doc = "API parts for the Tasks List API"]
pub enum TasksListParts {
    #[doc = "No parts"]
    None,
}
#[cfg(feature = "experimental-apis")]
impl TasksListParts {
    #[doc = "Builds a relative URL path to the Tasks List API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TasksListParts::None => "/_tasks".into(),
        }
    }
}
#[doc = "Builder for the [Tasks List API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nReturns a list of tasks."]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
#[derive(Clone, Debug)]
pub struct TasksList<'a, 'b> {
    transport: &'a Transport,
    parts: TasksListParts,
    actions: Option<&'b [&'b str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    group_by: Option<GroupBy>,
    headers: HeaderMap,
    human: Option<bool>,
    nodes: Option<&'b [&'b str]>,
    parent_task_id: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
#[cfg(feature = "experimental-apis")]
impl<'a, 'b> TasksList<'a, 'b> {
    #[doc = "Creates a new instance of [TasksList]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TasksList {
            transport,
            parts: TasksListParts::None,
            headers,
            actions: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            group_by: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_completion: None,
        }
    }
    #[doc = "A comma-separated list of actions that should be returned. Leave empty to return all."]
    pub fn actions(mut self, actions: &'b [&'b str]) -> Self {
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
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Group tasks by nodes or parent/child relationships"]
    pub fn group_by(mut self, group_by: GroupBy) -> Self {
        self.group_by = Some(group_by);
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
    #[doc = "A comma-separated list of node IDs or names to limit the returned information; use `_local` to return information from the node you're connecting to, leave empty to get information from all nodes"]
    pub fn nodes(mut self, nodes: &'b [&'b str]) -> Self {
        self.nodes = Some(nodes);
        self
    }
    #[doc = "Return tasks with specified parent task id (node_id:task_number). Set to -1 to return all."]
    pub fn parent_task_id(mut self, parent_task_id: &'b str) -> Self {
        self.parent_task_id = Some(parent_task_id);
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
    #[doc = "Explicit operation timeout"]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Wait for the matching tasks to complete (default: false)"]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Tasks List API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                actions: Option<&'b [&'b str]>,
                detailed: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                group_by: Option<GroupBy>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                nodes: Option<&'b [&'b str]>,
                parent_task_id: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                group_by: self.group_by,
                human: self.human,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "Namespace client for Tasks APIs"]
#[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
#[cfg(feature = "experimental-apis")]
pub struct Tasks<'a> {
    transport: &'a Transport,
}
#[cfg(feature = "experimental-apis")]
impl<'a> Tasks<'a> {
    #[doc = "Creates a new instance of [Tasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Tasks Cancel API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nCancels a task, if it can be cancelled through an API."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn cancel<'b>(&'a self, parts: TasksCancelParts<'b>) -> TasksCancel<'a, 'b, ()> {
        TasksCancel::new(self.transport(), parts)
    }
    #[doc = "[Tasks Get API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nReturns information about a task."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn get<'b>(&'a self, parts: TasksGetParts<'b>) -> TasksGet<'a, 'b> {
        TasksGet::new(self.transport(), parts)
    }
    #[doc = "[Tasks List API](https://www.elastic.co/guide/en/elasticsearch/reference/7.11/tasks.html)\n\nReturns a list of tasks."]
    #[doc = "&nbsp;\n# Optional, experimental\nThis requires the `experimental-apis` feature. Can have breaking changes in future\nversions or might even be removed entirely.\n        "]
    #[cfg(feature = "experimental-apis")]
    pub fn list<'b>(&'a self) -> TasksList<'a, 'b> {
        TasksList::new(self.transport())
    }
}
#[cfg(feature = "experimental-apis")]
impl Elasticsearch {
    #[doc = "Creates a namespace client for Tasks APIs"]
    pub fn tasks(&self) -> Tasks {
        Tasks::new(self.transport())
    }
}
