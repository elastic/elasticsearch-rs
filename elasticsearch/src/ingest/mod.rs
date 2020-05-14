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
//! Ingest APIs
//!
//! [Manage ingest pipelines](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-apis.html).
//! Ingest pipelines can be used on a node with the `ingest` role to
//! pre-process documents before indexing, to apply transformations and enrich data. Transformations are performed
//! by [processors](https://www.elastic.co/guide/en/elasticsearch/reference/master/ingest-processors.html)
//! in the pipeline, and can include such operations as
//!
//! - add, remove and append fields within the document
//! - point documents to the right time-based index based on a timestamp within the document
//! - extract details from fields with known formats and add new fields with extracted data
//!
//! and many more.
//!
//! All nodes enable ingest by default, so any node can handle ingest tasks. Ingest pipelines can
//! be conditionally executed, and failures within pipelines can be explicitly handled by defining
//! processors to execute in the event of failure.

pub use super::generated::namespace_clients::ingest::*;
