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
//! Licensing APIs
//!
//! [Manage Elastic Stack license](https://www.elastic.co/guide/en/elasticsearch/reference/master/licensing-apis.html), including
//!
//! - Retrieve, update or delete a license
//! - Start a 30 day trial of the Platinum license features
//! - Start indefinite use of the Basic license features
//! - Get the status of trial and basic license features

pub use super::generated::namespace_clients::license::*;
