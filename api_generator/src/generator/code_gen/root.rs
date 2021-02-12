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
use crate::generator::{
    code_gen::{request::request_builder::RequestBuilder, *},
    *,
};
use inflector::Inflector;
use quote::Tokens;
use std::path::PathBuf;

/// Generates the source code for the methods on the root of Elasticsearch
pub fn generate(api: &Api, docs_dir: &PathBuf) -> Result<String, failure::Error> {
    let mut tokens = Tokens::new();
    tokens.append(use_declarations());

    // AST for builder structs and methods
    let (builders, methods): (Vec<Tokens>, Vec<Tokens>) = api
        .root
        .endpoints()
        .iter()
        .map(|(name, endpoint)| {
            let builder_name = name.to_pascal_case();
            RequestBuilder::new(
                docs_dir,
                "Elasticsearch",
                name,
                &builder_name,
                &api.common_params,
                endpoint,
                true,
            )
            .build()
        })
        .unzip();

    tokens.append(quote!(
        #(#builders)*

        impl Elasticsearch {
            #(#methods)*
        }
    ));

    let generated = tokens.to_string();
    Ok(generated)
}
