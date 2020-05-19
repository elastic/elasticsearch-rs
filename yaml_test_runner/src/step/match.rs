use super::Step;
use crate::step::{clean_regex, Expr};
use quote::{ToTokens, Tokens};
use yaml_rust::{Yaml, YamlEmitter};
use crate::regex::*;

pub struct Match {
    pub expr: Expr,
    value: Yaml,
}

impl From<Match> for Step {
    fn from(m: Match) -> Self {
        Step::Match(m)
    }
}

impl Match {
    pub fn try_parse(yaml: &Yaml) -> Result<Match, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k.as_str().unwrap().trim();
        Ok(Match {
            expr: expr.into(),
            value: v.clone(),
        })
    }
}

impl ToTokens for Match {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let expr = self.expr.expression();

        match &self.value {
            Yaml::String(s) => {
                if s.starts_with('/') {
                    let s = clean_regex(s);
                    if self.expr.is_body() {
                        tokens.append(quote! {
                            assert_regex_match!(&text, #s, true);
                        });
                    } else {
                        let ident = syn::Ident::from(expr.as_str());
                        tokens.append(quote! {
                            assert_regex_match!(json#ident.as_str().unwrap(), #s, true);
                        });
                    }
                } else {
                    let ident = syn::Ident::from(expr.as_str());

                    // handle set values
                    if s.starts_with('$') {
                        let t = {
                            let s = s
                                .trim_start_matches('$')
                                .trim_start_matches('{')
                                .trim_end_matches('}');
                            syn::Ident::from(s)
                        };

                        tokens.append(quote! {
                            assert_match!(json#ident, json!(#t));
                        });
                    } else {
                        tokens.append(quote! {
                            assert_match!(json#ident, json!(#s));
                        })
                    };
                }
            }
            Yaml::Integer(i) => {
                if self.expr.is_body() {
                    panic!("match on $body with i64");
                } else {
                    let ident = syn::Ident::from(expr.as_str());
                    tokens.append(quote! {
                        assert_numeric_match!(json#ident, #i);
                    });
                }
            }
            Yaml::Real(r) => {
                let f = r.parse::<f64>().unwrap();
                if self.expr.is_body() {
                    panic!("match on $body with f64");
                } else {
                    let ident = syn::Ident::from(expr.as_str());
                    tokens.append(quote! {
                        assert_match!(json#ident, json!(#f));
                    });
                }
            }
            Yaml::Null => {
                if self.expr.is_body() {
                    tokens.append(quote! {
                        assert!(text.is_empty(), "expected response to be null (empty) but was {}", &text);
                    });
                } else {
                    let ident = syn::Ident::from(expr.as_str());
                    tokens.append(quote! {
                        assert_null!(json#ident);
                    });
                }
            }
            Yaml::Boolean(b) => {
                if self.expr.is_body() {
                    panic!("match on $body with bool");
                } else {
                    let ident = syn::Ident::from(expr.as_str());
                    tokens.append(quote! {
                        assert_match!(json#ident, json!(#b));
                    });
                }
            }
            yaml if yaml.is_array() || yaml.as_hash().is_some() => {
                let mut s = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut s);
                    emitter.dump(yaml).unwrap();
                }

                let value: serde_json::Value = serde_yaml::from_str(&s).unwrap();

                let json = {
                    let mut json = value.to_string();
                    json = replace_set_quoted_delimited(json);
                    json = replace_set_delimited(json);
                    json = replace_set(json);
                    json = replace_i64(json);
                    syn::Ident::from(json)
                };

                if self.expr.is_body() || self.expr.is_empty() {
                    tokens.append(quote! {
                        assert_match!(json, json!(#json));
                    });
                } else {
                    let ident = syn::Ident::from(expr.as_str());
                    tokens.append(quote! {
                        assert_match!(json#ident, json!(#json));
                    });
                }
            }
            yaml => {
                panic!("Bad yaml value {:?}", &yaml);
            }
        }
    }
}
