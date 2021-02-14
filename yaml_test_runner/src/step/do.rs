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
use super::{ok_or_accumulate, Step};
use crate::regex::{clean_regex, *};
use api_generator::generator::{Api, ApiEndpoint, TypeKind};
use inflector::Inflector;
use itertools::Itertools;
use quote::{ToTokens, Tokens};
use std::collections::BTreeMap;
use yaml_rust::{Yaml, YamlEmitter};

/// A catch expression on a do step
pub struct Catch(String);

impl Catch {
    fn needs_response_body(&self) -> bool {
        self.0.starts_with('/')
    }
}

impl ToTokens for Catch {
    fn to_tokens(&self, tokens: &mut Tokens) {
        fn http_status_code(status_code: u16, tokens: &mut Tokens) {
            tokens.append(quote! {
                assert_response_status_code!(response, #status_code);
            });
        }

        match self.0.as_ref() {
            "bad_request" => http_status_code(400, tokens),
            "unauthorized" => http_status_code(401, tokens),
            "forbidden" => http_status_code(403, tokens),
            "missing" => http_status_code(404, tokens),
            "request_timeout" => http_status_code(408, tokens),
            "conflict" => http_status_code(409, tokens),
            "request" => {
                tokens.append(quote! {
                    assert_request_status_code!(response.status_code());
                });
            }
            "unavailable" => http_status_code(503, tokens),
            "param" => {
                // Not possible to pass a bad param to the client so ignore.
            }
            s => {
                let t = clean_regex(s);
                tokens.append(quote! {
                    assert_regex_match!(&text, #t);
                });
            }
        }
    }
}

pub struct Do {
    api_call: ApiCall,
    warnings: Vec<String>,
    allowed_warnings: Vec<String>,
    catch: Option<Catch>,
}

impl ToTokens for Do {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let _ = self.to_tokens(false, tokens);
    }
}

impl From<Do> for Step {
    fn from(d: Do) -> Self {
        Step::Do(d)
    }
}

impl Do {
    pub fn to_tokens(&self, mut read_response: bool, tokens: &mut Tokens) -> bool {
        self.api_call.to_tokens(tokens);

        // Filter out [types removal] warnings in all cases, same as the java runner. This should
        // really be in the yaml tests themselves
        if !self.warnings.is_empty() {
            tokens.append(quote! {
                let warnings: Vec<&str> = response
                    .warning_headers()
                    .filter(|w| !w.starts_with("[types removal]"))
                    .collect();
            });
            for warning in &self.warnings {
                tokens.append(quote! {
                    assert_warnings_contain!(warnings, #warning);
                });
            }
        } else if !self.allowed_warnings.is_empty() {
            let allowed = &self.allowed_warnings;
            tokens.append(quote! {
                let allowed_warnings = vec![#(#allowed),*];
                let warnings: Vec<&str> = response.warning_headers()
                    .filter(|w| !w.starts_with("[types removal]") && !allowed_warnings.iter().any(|a| w.contains(a)))
                    .collect();
                assert_warnings_is_empty!(warnings);
            });
        }

        if let Some(c) = &self.catch {
            if !read_response && c.needs_response_body() {
                read_response = true;
                tokens.append(quote! {
                    let (method, status_code, text, json) = client::read_response(response).await?;
                });
            }
            c.to_tokens(tokens);
        }

        if let Some(i) = &self.api_call.ignore {
            tokens.append(quote! {
                assert_response_success_or!(response, #i);
            });
        }

        read_response
    }

    pub fn try_parse(api: &Api, yaml: &Yaml) -> Result<Do, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let mut call: Option<(&str, &Yaml)> = None;
        let mut headers = BTreeMap::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut allowed_warnings: Vec<String> = Vec::new();
        let mut catch = None;

        fn to_string_vec(v: &Yaml) -> Vec<String> {
            v.as_vec()
                .map(|a| a.iter().map(|y| y.as_str().unwrap().to_string()).collect())
                .unwrap()
        }

        let results: Vec<Result<(), failure::Error>> = hash
            .iter()
            .map(|(k, v)| {
                let key = k.as_str().ok_or_else(|| {
                    failure::err_msg(format!("expected string but found {:?}", k))
                })?;

                match key {
                    "headers" => {
                        let hash = v.as_hash().ok_or_else(|| {
                            failure::err_msg(format!("expected hash but found {:?}", v))
                        })?;
                        for (hk, hv) in hash.iter() {
                            let h = hk.as_str().ok_or_else(|| {
                                failure::err_msg(format!("expected string but found {:?}", hk))
                            })?;
                            let v = hv.as_str().ok_or_else(|| {
                                failure::err_msg(format!("expected string but found {:?}", hv))
                            })?;
                            headers.insert(h.into(), v.into());
                        }
                        Ok(())
                    }
                    "catch" => {
                        catch = v.as_str().map(|s| Catch(s.to_string()));
                        Ok(())
                    }
                    "node_selector" => Ok(()),
                    "warnings" => {
                        warnings = to_string_vec(v);
                        Ok(())
                    }
                    "allowed_warnings" => {
                        allowed_warnings = to_string_vec(v);
                        Ok(())
                    }
                    api_call => {
                        call = Some((api_call, v));
                        Ok(())
                    }
                }
            })
            .collect();

        ok_or_accumulate(&results)?;

        let (call, value) = call.ok_or_else(|| failure::err_msg("no API found in do"))?;
        let endpoint = api
            .endpoint_for_api_call(call)
            .ok_or_else(|| failure::err_msg(format!(r#"no API found for "{}""#, call)))?;
        let api_call = ApiCall::try_from(api, endpoint, value, headers)?;

        Ok(Do {
            api_call,
            catch,
            warnings,
            allowed_warnings,
        })
    }

    pub fn namespace(&self) -> Option<&String> {
        self.api_call.namespace.as_ref()
    }
}

/// The components of an API call
pub struct ApiCall {
    pub namespace: Option<String>,
    function: syn::Ident,
    parts: Option<Tokens>,
    params: Option<Tokens>,
    headers: BTreeMap<String, String>,
    body: Option<Tokens>,
    ignore: Option<u16>,
}

impl ToTokens for ApiCall {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let function = &self.function;
        let parts = &self.parts;
        let params = &self.params;
        let body = &self.body;

        let headers: Vec<Tokens> = self
            .headers
            .iter()
            .map(|(k, v)| {
                // header names **must** be lowercase to satisfy Header lib
                let k = k.to_lowercase();

                // handle "set" value in headers
                if let Some(c) = SET_DELIMITED_REGEX.captures(v) {
                    let token = syn::Ident::from(c.get(1).unwrap().as_str());
                    let replacement = SET_DELIMITED_REGEX.replace_all(v, "{}");
                    quote! { .header(
                        HeaderName::from_static(#k),
                        HeaderValue::from_str(format!(#replacement, #token.as_str().unwrap()).as_ref())?)
                    }
                } else {
                    quote! { .header(
                        HeaderName::from_static(#k),
                        HeaderValue::from_static(#v))
                    }
                }
            })
            .collect();

        tokens.append(quote! {
            let response = client.#function(#parts)
                #(#headers)*
                #params
                #body
                .send()
                .await?;
        });
    }
}

impl ApiCall {
    /// Try to create an API call
    pub fn try_from(
        api: &Api,
        endpoint: &ApiEndpoint,
        yaml: &Yaml,
        headers: BTreeMap<String, String>,
    ) -> Result<ApiCall, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let mut parts: Vec<(&str, &Yaml)> = vec![];
        let mut params: Vec<(&str, &Yaml)> = vec![];
        let mut body: Option<Tokens> = None;
        let mut ignore: Option<u16> = None;

        // work out what's a URL part and what's a param in the supplied
        // arguments for the API call
        for (k, v) in hash.iter() {
            match k.as_str().unwrap() {
                "body" => body = Self::generate_body(endpoint, v)?,
                "ignore" => {
                    ignore = match v.as_i64() {
                        Some(i) => Some(i as u16),
                        // handle ignore as an array of i64
                        None => Some(v.as_vec().unwrap()[0].as_i64().unwrap() as u16),
                    }
                }
                key if endpoint.params.contains_key(key) || api.common_params.contains_key(key) => {
                    params.push((key, v))
                }
                key => parts.push((key, v)),
            }
        }

        let api_call = endpoint.full_name.as_ref().unwrap();
        let parts = Self::generate_parts(api_call, endpoint, &parts)?;
        let params = Self::generate_params(api, endpoint, &params)?;
        let function = syn::Ident::from(api_call.replace(".", "()."));
        let namespace: Option<String> = if api_call.contains('.') {
            let namespaces: Vec<&str> = api_call.splitn(2, '.').collect();
            Some(namespaces[0].to_string())
        } else {
            None
        };

        Ok(ApiCall {
            namespace,
            function,
            parts,
            params,
            headers,
            body,
            ignore,
        })
    }

    fn generate_enum(
        enum_name: &str,
        variant: &str,
        options: &[serde_json::Value],
    ) -> Result<Tokens, failure::Error> {
        if !variant.is_empty() && !options.contains(&serde_json::Value::String(variant.to_owned()))
        {
            return Err(failure::err_msg(format!(
                "options {:?} does not contain value {}",
                &options, variant
            )));
        }

        let e: String = enum_name.to_pascal_case();
        let enum_name = syn::Ident::from(e.as_str());
        let variant = if variant.is_empty() {
            // TODO: Should we simply omit empty Refresh tests?
            if e == "Refresh" {
                syn::Ident::from("True")
            } else if e == "Size" {
                syn::Ident::from("Unspecified")
            } else {
                return Err(failure::err_msg(format!(
                    "unhandled empty value for {}",
                    &e
                )));
            }
        } else {
            syn::Ident::from(variant.to_pascal_case())
        };

        Ok(quote!(#enum_name::#variant))
    }

    fn generate_params(
        api: &Api,
        endpoint: &ApiEndpoint,
        params: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        match params.len() {
            0 => Ok(None),
            _ => {
                let mut tokens = Tokens::new();
                for (n, v) in params {
                    let param_ident =
                        syn::Ident::from(api_generator::generator::code_gen::valid_name(n));

                    let ty = match endpoint.params.get(*n) {
                        Some(t) => Ok(t),
                        None => match api.common_params.get(*n) {
                            Some(t) => Ok(t),
                            None => Err(failure::err_msg(format!(r#"no param found for "{}""#, n))),
                        },
                    }?;

                    let kind = &ty.ty;

                    match v {
                        Yaml::String(ref s) => {
                            let is_set_value = s.starts_with('$');

                            match kind {
                                TypeKind::Enum => {
                                    if n == &"expand_wildcards" {
                                        // expand_wildcards might be defined as a comma-separated
                                        // string. e.g.
                                        let idents: Vec<Result<Tokens, failure::Error>> = s
                                            .split(',')
                                            .collect::<Vec<_>>()
                                            .iter()
                                            .map(|e| Self::generate_enum(n, e, &ty.options))
                                            .collect();

                                        match ok_or_accumulate(&idents) {
                                            Ok(_) => {
                                                let idents: Vec<Tokens> = idents
                                                    .into_iter()
                                                    .filter_map(Result::ok)
                                                    .collect();

                                                tokens.append(quote! {
                                                    .#param_ident(&[#(#idents),*])
                                                });
                                            }
                                            Err(e) => return Err(failure::err_msg(e)),
                                        }
                                    } else {
                                        let e = Self::generate_enum(n, s.as_str(), &ty.options)?;
                                        tokens.append(quote! {
                                            .#param_ident(#e)
                                        });
                                    }
                                }
                                TypeKind::List => {
                                    let values: Vec<&str> = s.split(',').collect();
                                    tokens.append(quote! {
                                        .#param_ident(&[#(#values),*])
                                    })
                                }
                                TypeKind::Boolean => match s.parse::<bool>() {
                                    Ok(b) => tokens.append(quote! {
                                        .#param_ident(#b)
                                    }),
                                    Err(e) => {
                                        return Err(failure::err_msg(format!(
                                            r#"cannot parse bool from "{}" for param "{}", {}"#,
                                            s,
                                            n,
                                            e.to_string()
                                        )))
                                    }
                                },
                                TypeKind::Double => match s.parse::<f64>() {
                                    Ok(f) => tokens.append(quote! {
                                        .#param_ident(#f)
                                    }),
                                    Err(e) => {
                                        return Err(failure::err_msg(format!(
                                            r#"cannot parse f64 from "{}" for param "{}", {}"#,
                                            s,
                                            n,
                                            e.to_string()
                                        )))
                                    }
                                },
                                TypeKind::Integer => {
                                    if is_set_value {
                                        let set_value = Self::from_set_value(s);
                                        tokens.append(quote! {
                                           .#param_ident(#set_value.as_i64().unwrap() as i32)
                                        });
                                    } else {
                                        match s.parse::<i32>() {
                                            Ok(i) => tokens.append(quote! {
                                                .#param_ident(#i)
                                            }),
                                            Err(e) => {
                                                return Err(failure::err_msg(format!(
                                                    r#"cannot parse i32 from "{}" for param "{}", {}"#,
                                                    s,
                                                    n,
                                                    e.to_string()
                                                )))
                                            }
                                        }
                                    }
                                }
                                TypeKind::Number | TypeKind::Long => {
                                    if is_set_value {
                                        let set_value = Self::from_set_value(s);
                                        tokens.append(quote! {
                                           .#param_ident(#set_value.as_i64().unwrap())
                                        });
                                    } else {
                                        let i = s.parse::<i64>()?;
                                        tokens.append(quote! {
                                            .#param_ident(#i)
                                        });
                                    }
                                }
                                _ => {
                                    // handle set values
                                    let t = if is_set_value {
                                        let set_value = Self::from_set_value(s);
                                        quote! { #set_value.as_str().unwrap() }
                                    } else {
                                        quote! { #s }
                                    };

                                    tokens.append(quote! {
                                        .#param_ident(#t)
                                    })
                                }
                            }
                        }
                        Yaml::Boolean(ref b) => match kind {
                            TypeKind::Enum => {
                                let enum_name = syn::Ident::from(n.to_pascal_case());
                                let variant = syn::Ident::from(b.to_string().to_pascal_case());
                                tokens.append(quote! {
                                    .#param_ident(#enum_name::#variant)
                                })
                            }
                            TypeKind::List => {
                                // TODO: _source filter can be true|false|list of strings
                                let s = b.to_string();
                                tokens.append(quote! {
                                    .#param_ident(&[#s])
                                })
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#b)
                                });
                            }
                        },
                        Yaml::Integer(ref i) => match kind {
                            TypeKind::String => {
                                let s = i.to_string();
                                tokens.append(quote! {
                                    .#param_ident(#s)
                                })
                            }
                            TypeKind::Integer => {
                                // yaml-rust parses all as i64
                                let int = *i as i32;
                                tokens.append(quote! {
                                    .#param_ident(#int)
                                });
                            }
                            TypeKind::Float => {
                                // yaml-rust parses all as i64
                                let f = *i as f32;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            TypeKind::Double => {
                                // yaml-rust parses all as i64
                                let f = *i as f64;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#i)
                                });
                            }
                        },
                        Yaml::Array(arr) => {
                            // only support param string arrays
                            let result: Vec<&String> = arr
                                .iter()
                                .map(|i| match i {
                                    Yaml::String(s) => Ok(s),
                                    y => Err(failure::err_msg(format!(
                                        "unsupported array value {:?}",
                                        y
                                    ))),
                                })
                                .filter_map(Result::ok)
                                .collect();

                            if n == &"expand_wildcards" {
                                let result: Vec<Result<Tokens, failure::Error>> = result
                                    .iter()
                                    .map(|s| Self::generate_enum(n, s.as_str(), &ty.options))
                                    .collect();

                                match ok_or_accumulate(&result) {
                                    Ok(_) => {
                                        let result: Vec<Tokens> =
                                            result.into_iter().filter_map(Result::ok).collect();

                                        tokens.append(quote! {
                                            .#param_ident(&[#(#result),*])
                                        });
                                    }
                                    Err(e) => return Err(failure::err_msg(e)),
                                }
                            } else {
                                tokens.append(quote! {
                                    .#param_ident(&[#(#result),*])
                                });
                            }
                        }
                        Yaml::Real(r) => match kind {
                            TypeKind::Long | TypeKind::Number => {
                                let f = r.parse::<f64>()?;
                                tokens.append(quote! {
                                    .#param_ident(#f as i64)
                                });
                            }
                            _ => {
                                let f = r.parse::<f64>()?;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                        },
                        _ => println!("unsupported value {:?} for param {}", v, n),
                    }
                }

                Ok(Some(tokens))
            }
        }
    }

    fn from_set_value(s: &str) -> Tokens {
        // check if the entire string is a token
        if s.starts_with('$') {
            let ident = syn::Ident::from(
                s.trim_start_matches('$')
                    .trim_start_matches('{')
                    .trim_end_matches('}'),
            );
            quote! { #ident }
        } else {
            // only part of the string is a token, so substitute
            let token = syn::Ident::from(
                SET_DELIMITED_REGEX
                    .captures(s)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str(),
            );
            let replacement = SET_DELIMITED_REGEX.replace_all(s, "{}");
            quote! { Value::String(format!(#replacement, #token.as_str().unwrap())) }
        }
    }

    fn generate_parts(
        api_call: &str,
        endpoint: &ApiEndpoint,
        parts: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        // TODO: ideally, this should share the logic from EnumBuilder
        let enum_name = {
            let name = api_call.to_pascal_case().replace(".", "");
            syn::Ident::from(format!("{}Parts", name))
        };

        // Enum variants containing no URL parts where there is only a single API URL,
        // are not required to be passed in the API.
        //
        // Also, short circuit for tests where the only parts specified are null
        // e.g. security API test. It seems these should simply omit the value though...
        if parts.is_empty() || parts.iter().all(|(_, v)| v.is_null()) {
            let param_counts = endpoint
                .url
                .paths
                .iter()
                .map(|p| p.path.params().len())
                .collect::<Vec<usize>>();

            // check there's actually a None value
            if !param_counts.contains(&0) {
                return Err(failure::err_msg(format!(
                    r#"no path for "{}" API with no url parts"#,
                    api_call
                )));
            }

            return match endpoint.url.paths.len() {
                1 => Ok(None),
                _ => Ok(Some(quote!(#enum_name::None))),
            };
        }

        let path = match endpoint.url.paths.len() {
            1 => {
                let path = &endpoint.url.paths[0];
                if path.path.params().len() == parts.len() {
                    Some(path)
                } else {
                    None
                }
            }
            _ => {
                // get the matching path parts
                let matching_path_parts = endpoint
                    .url
                    .paths
                    .iter()
                    .filter(|path| {
                        let p = path.path.params();
                        if p.len() != parts.len() {
                            return false;
                        }

                        let contains = parts
                            .iter()
                            .filter_map(|i| if p.contains(&i.0) { Some(()) } else { None })
                            .collect::<Vec<_>>();
                        contains.len() == parts.len()
                    })
                    .collect::<Vec<_>>();

                match matching_path_parts.len() {
                    0 => None,
                    _ => Some(matching_path_parts[0]),
                }
            }
        }
        .ok_or_else(|| {
            failure::err_msg(format!(
                r#"no path for "{}" API with url parts {:?}"#,
                &api_call, parts
            ))
        })?;

        let path_parts = path.path.params();
        let variant_name = {
            let v = path_parts
                .iter()
                .map(|k| k.to_pascal_case())
                .collect::<Vec<_>>()
                .join("");
            syn::Ident::from(v)
        };

        let part_tokens: Vec<Result<Tokens, failure::Error>> = parts
            .iter()
            // don't rely on URL parts being ordered in the yaml test in the same order as specified
            // in the REST spec.
            .sorted_by(|(p, _), (p2, _)| {
                let f = path_parts.iter().position(|x| x == p).unwrap();
                let s = path_parts.iter().position(|x| x == p2).unwrap();
                f.cmp(&s)
            })
            .map(|(p, v)| {
                let ty = path.parts.get(*p).ok_or_else(|| {
                    failure::err_msg(format!(
                        r#"no url part found for "{}" in {}"#,
                        p, &path.path
                    ))
                })?;

                match v {
                    Yaml::String(s) => {
                        let is_set_value = s.starts_with('$') || s.contains("${");

                        match ty.ty {
                            TypeKind::List => {
                                let values: Vec<Tokens> = s
                                    .split(',')
                                    .map(|s| {
                                        if is_set_value {
                                            let set_value = Self::from_set_value(s);
                                            quote! { #set_value.as_str().unwrap() }
                                        } else {
                                            quote! { #s }
                                        }
                                    })
                                    .collect();
                                Ok(quote! { &[#(#values),*] })
                            }
                            TypeKind::Long => {
                                if is_set_value {
                                    let set_value = Self::from_set_value(s);
                                    Ok(quote! { #set_value.as_i64().unwrap() })
                                } else {
                                    let l = s.parse::<i64>().unwrap();
                                    Ok(quote! { #l })
                                }
                            }
                            _ => {
                                if is_set_value {
                                    let set_value = Self::from_set_value(s);
                                    Ok(quote! { #set_value.as_str().unwrap() })
                                } else {
                                    Ok(quote! { #s })
                                }
                            }
                        }
                    }
                    Yaml::Boolean(b) => {
                        let s = b.to_string();
                        Ok(quote! { #s })
                    }
                    Yaml::Integer(i) => match ty.ty {
                        TypeKind::Long => Ok(quote! { #i }),
                        _ => {
                            let s = i.to_string();
                            Ok(quote! { #s })
                        }
                    },
                    Yaml::Array(arr) => {
                        // only support param string arrays
                        let result: Vec<_> = arr
                            .iter()
                            .map(|i| match i {
                                Yaml::String(s) => Ok(s),
                                y => Err(failure::err_msg(format!(
                                    "unsupported array value {:?}",
                                    y
                                ))),
                            })
                            .collect();

                        match ok_or_accumulate(&result) {
                            Ok(_) => {
                                let result: Vec<_> =
                                    result.into_iter().filter_map(Result::ok).collect();

                                match ty.ty {
                                    // Some APIs specify a part is a string in the REST API spec
                                    // but is really a list, which is what a YAML test might pass
                                    // e.g. security.get_role_mapping.
                                    // see https://github.com/elastic/elasticsearch/pull/53207
                                    TypeKind::String => {
                                        let s = result.iter().join(",");
                                        Ok(quote! { #s })
                                    }
                                    _ => Ok(quote! { &[#(#result),*] }),
                                }
                            }
                            Err(e) => Err(failure::err_msg(e)),
                        }
                    }
                    _ => Err(failure::err_msg(format!("unsupported value {:?}", v))),
                }
            })
            .collect();

        match ok_or_accumulate(&part_tokens) {
            Ok(_) => {
                let part_tokens: Vec<Tokens> =
                    part_tokens.into_iter().filter_map(Result::ok).collect();
                Ok(Some(
                    quote! { #enum_name::#variant_name(#(#part_tokens),*) },
                ))
            }
            Err(e) => Err(failure::err_msg(e)),
        }
    }

    /// Creates the body function call from a YAML value.
    ///
    /// When reading a body from the YAML test, it'll be converted to a Yaml variant,
    /// usually a Hash. To get the JSON representation back requires converting
    /// back to JSON
    fn generate_body(endpoint: &ApiEndpoint, v: &Yaml) -> Result<Option<Tokens>, failure::Error> {
        match v {
            Yaml::Null => Ok(None),
            Yaml::String(s) => {
                let json = {
                    let json = replace_set(s);
                    replace_i64(json)
                };
                if endpoint.supports_nd_body() {
                    // a newline delimited API body may be expressed
                    // as a scalar string literal style where line breaks are significant (using |)
                    // or where lines breaks are folded to an empty space unless it ends on an
                    // empty or a more-indented line (using >)
                    // see https://yaml.org/spec/1.2/spec.html#id2760844
                    //
                    // need to trim the trailing newline to be able to differentiate...
                    let contains_newlines = json.trim_end_matches('\n').contains('\n');
                    let split = if contains_newlines {
                        json.split('\n').collect::<Vec<_>>()
                    } else {
                        json.split(char::is_whitespace).collect::<Vec<_>>()
                    };

                    let values: Vec<Tokens> = split
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            let ident = syn::Ident::from(s);
                            quote! { JsonBody::from(json!(#ident)) }
                        })
                        .collect();
                    Ok(Some(quote!(.body(vec![#(#values),*]))))
                } else {
                    let ident = syn::Ident::from(json);
                    Ok(Some(quote!(.body(json!{#ident}))))
                }
            }
            _ => {
                let mut s = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut s);
                    emitter.dump(v).unwrap();
                }

                if endpoint.supports_nd_body() {
                    let values: Vec<serde_json::Value> = serde_yaml::from_str(&s)?;
                    let json: Vec<Tokens> = values
                        .iter()
                        .map(|value| {
                            let mut json = serde_json::to_string(&value).unwrap();
                            if value.is_string() {
                                json = replace_set(&json);
                                let ident = syn::Ident::from(json);
                                quote!(#ident)
                            } else {
                                json = replace_set(json);
                                json = replace_i64(json);
                                let ident = syn::Ident::from(json);
                                quote!(JsonBody::from(json!(#ident)))
                            }
                        })
                        .collect();
                    Ok(Some(quote!(.body(vec![ #(#json),* ]))))
                } else {
                    let value: serde_json::Value = serde_yaml::from_str(&s)?;
                    let mut json = serde_json::to_string_pretty(&value)?;
                    json = replace_set(json);
                    json = replace_i64(json);
                    let ident = syn::Ident::from(json);

                    Ok(Some(quote!(.body(json!{#ident}))))
                }
            }
        }
    }
}
