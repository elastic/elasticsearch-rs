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
use crate::step::{json_string_from_yaml, Expr};
use quote::{ToTokens, Tokens};
use yaml_rust::Yaml;

pub struct Contains {
    expr: Expr,
    value: Yaml,
}

impl From<Contains> for Step {
    fn from(contains: Contains) -> Self {
        Step::Contains(contains)
    }
}

impl Contains {
    pub fn try_parse(yaml: &Yaml) -> Result<Contains, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k.as_str().unwrap().trim();
        Ok(Contains {
            expr: expr.into(),
            value: v.clone(),
        })
    }
}

impl ToTokens for Contains {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let expr = self.expr.expression();
        let ident = syn::Ident::from(expr.as_str());

        match &self.value {
            Yaml::Real(r) => {
                let f = r.parse::<f64>().unwrap();
                tokens.append(quote! {
                    assert_contains!(json#ident, json!(#f));
                });
            }
            Yaml::Integer(i) => {
                tokens.append(quote! {
                    assert_contains!(json#ident, json!(#i));
                });
            }
            Yaml::String(s) => {
                tokens.append(quote! {
                    assert_contains!(json#ident, json!(#s));
                });
            }
            Yaml::Boolean(b) => {
                tokens.append(quote! {
                    assert_contains!(json#ident, json!(#b));
                });
            }
            yaml if yaml.is_array() || yaml.as_hash().is_some() => {
                let json = {
                    let s = json_string_from_yaml(yaml);
                    syn::Ident::from(s)
                };

                tokens.append(quote! {
                    assert_contains!(json#ident, json!(#json));
                });
            }
            yaml => {
                panic!("Bad yaml value {:?}", &yaml);
            }
        }
    }
}
