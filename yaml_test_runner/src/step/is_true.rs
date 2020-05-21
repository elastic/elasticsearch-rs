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
use super::Step;
use crate::step::Expr;
use quote::{ToTokens, Tokens};
use yaml_rust::Yaml;

pub struct IsTrue {
    pub(crate) expr: Expr,
}

impl From<IsTrue> for Step {
    fn from(is_true: IsTrue) -> Self {
        Step::IsTrue(is_true)
    }
}

impl IsTrue {
    pub fn try_parse(yaml: &Yaml) -> Result<IsTrue, failure::Error> {
        let expr = yaml.as_str().ok_or_else(|| {
            failure::err_msg(format!("expected string key but found {:?}", &yaml))
        })?;

        Ok(IsTrue { expr: expr.into() })
    }
}

impl ToTokens for IsTrue {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if self.expr.is_body() {
            // for a HEAD request, the body is expected to be empty, so check the status code instead.
            tokens.append(quote! {
                match method {
                    Method::Head => assert!(status_code.is_success(), "expected successful response for HEAD request but was {}", status_code.as_u16()),
                    _ => assert!(!text.is_empty(), "expected value to be true (not empty) but was {}", &text),
                }
            });
        } else {
            let expr = self.expr.expression();
            let ident = syn::Ident::from(expr.as_str());
            tokens.append(quote! {
                assert_is_true!(&json#ident);
            });
        }
    }
}
