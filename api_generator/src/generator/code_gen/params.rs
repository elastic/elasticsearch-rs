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
use crate::generator::code_gen::stability_doc;
use crate::generator::*;
use inflector::Inflector;
use quote::Tokens;
use regex::Regex;

pub fn generate(api: &Api) -> anyhow::Result<String> {
    let mut tokens = quote!(
        use serde::{Serialize, Deserialize};
    );
    for e in &api.enums {
        generate_param(&mut tokens, e);
    }

    let generated = tokens.to_string();
    Ok(generated)
}

fn generate_display_impl(
    name: &syn::Ident,
    renames: &[String],
    variants: &[syn::Ident],
    cfg_attr: &Option<Tokens>,
) -> Tokens {
    let match_arms: Vec<Tokens> = renames
        .iter()
        .zip(variants.iter())
        .map(|(rename, variant)| quote!(#name::#variant => write!(f, #rename)))
        .collect();

    quote!(
        #cfg_attr
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    #(#match_arms),*
                }
            }
        }
    )
}

fn generate_param(tokens: &mut Tokens, e: &ApiEnum) {
    let name = syn::Ident::from(e.name.to_pascal_case());
    let (renames, variants): (Vec<String>, Vec<syn::Ident>) = e
        .values
        .iter()
        .map(|v| {
            if v.is_empty() {
                (v.to_owned(), syn::Ident::from("Unspecified"))
            } else if !v.contains('(') {
                (v.to_owned(), syn::Ident::from(v.to_pascal_case()))
            } else {
                lazy_static! {
                    static ref PARENS_REGEX: Regex = Regex::new(r"^(.*?)\s*\(.*?\)\s*$").unwrap();
                }
                if let Some(c) = PARENS_REGEX.captures(v) {
                    (
                        c.get(1).unwrap().as_str().to_owned(),
                        syn::Ident::from(c.get(1).unwrap().as_str().to_pascal_case()),
                    )
                } else {
                    (v.to_owned(), syn::Ident::from(v.to_pascal_case()))
                }
            }
        })
        .unzip();

    let doc = e.description.as_ref().map(code_gen::doc_escaped);
    let cfg_attr = e.stability.outer_cfg_attr();
    let cfg_doc = stability_doc(e.stability);

    let display_impl = generate_display_impl(&name, &renames, &variants, &cfg_attr);
    let generated_enum_tokens = quote!(
        #doc
        #cfg_doc
        #cfg_attr
        #[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
        pub enum #name {
            #(#[serde(rename = #renames)] #variants),*
        }
    );

    tokens.append(generated_enum_tokens);
    tokens.append(display_impl);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{ast_eq, ApiEnum, Stability};

    #[test]
    fn test_generate_param_enum() {
        let e = ApiEnum {
            name: "expand_wildcards".to_string(),
            description: Some("The type of index expansion to perform.".to_string()),
            values: vec!["open".to_string(), "closed".to_string(), "none".to_string()],
            stability: Stability::Stable,
        };

        let mut tokens = Tokens::new();
        generate_param(&mut tokens, &e);

        let expected = quote!(
            #[doc = "The type of index expansion to perform."]
            #[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy, Eq)]
            pub enum ExpandWildcards {
                #[serde(rename = "open")]
                Open,
                #[serde(rename = "closed")]
                Closed,
                #[serde(rename = "none")]
                None
            }

            impl ::std::fmt::Display for ExpandWildcards {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        ExpandWildcards::Open => write!(f, "open"),
                        ExpandWildcards::Closed => write!(f, "closed"),
                        ExpandWildcards::None => write!(f, "none")
                    }
                }
            }
        );

        ast_eq(expected, tokens);
    }
}
