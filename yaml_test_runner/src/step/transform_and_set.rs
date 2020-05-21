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
use inflector::Inflector;
use quote::{ToTokens, Tokens};
use yaml_rust::Yaml;

pub struct Transformation {
    #[allow(dead_code)]
    raw: String,
    function: String,
    exprs: Vec<Expr>,
}

impl Transformation {
    pub fn transform(&self) -> syn::Ident {
        let mut transform = String::new();
        transform.push_str(&self.function);
        transform.push('(');
        for expr in &self.exprs {
            transform.push_str("json");
            transform.push_str(expr.expression().as_str());
            transform.push_str(".as_str().unwrap()");
            transform.push(',');
        }
        transform.push(')');
        syn::Ident::from(transform.as_str())
    }
}

impl From<&str> for Transformation {
    fn from(t: &str) -> Self {
        let raw = t.to_string();
        let mut function = None;
        let mut exprs = Vec::new();
        let mut value = String::new();
        for ch in t.chars() {
            match ch {
                '#' => {
                    continue;
                }
                '(' => {
                    let name = format!("transform::{}", value.as_str().to_snake_case());
                    function = Some(name);
                    value = String::new();
                }
                ',' | ')' => {
                    let expr = value.trim();
                    exprs.push(Expr::new(expr));
                    value = String::new();
                }
                _ => {
                    value.push(ch);
                }
            }
        }

        Self {
            raw,
            function: function.unwrap(),
            exprs,
        }
    }
}

pub struct TransformAndSet {
    ident: syn::Ident,
    transformation: Transformation,
}

impl From<TransformAndSet> for Step {
    fn from(transform_and_set: TransformAndSet) -> Self {
        Step::TransformAndSet(transform_and_set)
    }
}

impl TransformAndSet {
    pub fn try_parse(yaml: &Yaml) -> Result<TransformAndSet, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();
        let ident = k
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string key but found {:?}", k)))?;

        let transformation = v
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string value but found {:?}", v)))?;

        Ok(TransformAndSet {
            ident: syn::Ident::from(ident),
            transformation: transformation.into(),
        })
    }
}

impl ToTokens for TransformAndSet {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let ident = &self.ident;
        let transform = &self.transformation.transform();
        tokens.append(quote! {
            let #ident = {
                let transform = #transform;
                json!(transform)
            };
        });
    }
}
