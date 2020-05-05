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
//! Index Lifecycle Management APIs
//!
//! Automate how [indices are managed over time](https://www.elastic.co/guide/en/elasticsearch/reference/master/index-lifecycle-management.html).
//! Rather than simply performing management actions on indices on a set schedule, actions can be based
//! on other factors such as shard size and performance requirements.
//!
//! Control how indices are handled as they age by attaching a lifecycle policy to the index
//! template used to create them. Update the policy to modify the lifecycle of both new
//! and existing indices.

pub use super::generated::namespace_clients::ilm::*;
