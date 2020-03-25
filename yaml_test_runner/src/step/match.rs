use super::Step;
use quote::{ToTokens, Tokens};
use yaml_rust::{yaml::Hash, Yaml};
use crate::step::BodyExpr;

pub struct Match {
    hash: Hash,
}

impl From<Match> for Step {
    fn from(m: Match) -> Self {
        Step::Match(m)
    }
}

impl BodyExpr for Match {}

impl Match {
    pub fn try_parse(yaml: &Yaml) -> Result<Match, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("Expected hash but found {:?}", yaml)))?;

        Ok(Match { hash: hash.clone() })
    }
}

impl ToTokens for Match {
    // TODO: Move this parsing out into Match::try_parse
    fn to_tokens(&self, tokens: &mut Tokens) {
        let (k, v) = self.hash.iter().next().unwrap();
        let key = k.as_str().unwrap().trim();
        let expr = self.body_expr(key);

        match v {
            Yaml::String(s) => {
                if s.starts_with('/') {
                    let s = s.trim().trim_matches('/');
                    if expr == "$body" {
                        tokens.append(quote! {
                            let string_response_body = serde_json::to_string(&response_body).unwrap();
                            let regex = regex::Regex::new(#s)?;
                            assert!(
                                regex.is_match(&string_response_body),
                                "expected $body:\n\n{}\n\nto match regex:\n\n{}",
                                &string_response_body,
                                #s
                            );
                        });
                    } else {
                        let ident = syn::Ident::from(expr.clone());
                        tokens.append(quote! {
                            let regex = regex::Regex::new(#s)?;
                            assert!(
                                regex.is_match(response_body#ident.as_str().unwrap()),
                                "expected value at {}:\n\n{}\n\nto match regex:\n\n{}",
                                #expr,
                                response_body#ident.as_str().unwrap(),
                                #s
                            );
                        });
                    }
                } else {
                    let ident = syn::Ident::from(expr.clone());

                    // handle set values
                    let t = if s.starts_with('$') {
                        let t = s.trim_start_matches('$')
                            .trim_start_matches('{')
                            .trim_end_matches('}');
                        let ident = syn::Ident::from(t);
                        quote!{ #ident.as_str().unwrap() }
                    } else {
                        quote! { #s }
                    };

                    tokens.append(quote! {
                        assert_eq!(
                            response_body#ident.as_str().unwrap(),
                            #t,
                            "expected value at {} to be {} but was {}",
                            #expr,
                            #t,
                            response_body#ident.as_str().unwrap()
                        );
                    })
                }
            }
            Yaml::Integer(i) => {
                if expr == "$body" {
                    panic!("match on $body with integer");
                } else {
                    let ident = syn::Ident::from(expr.clone());
                    tokens.append(quote! {
                        assert_eq!(
                            response_body#ident.as_i64().unwrap(),
                            #i,
                            "expected value at {} to be {} but was {}",
                            #expr,
                            #i,
                            response_body#ident.as_i64().unwrap()
                        );
                    });
                }
            }
            // TODO: handle hashes, etc.
            _ => {}
        }
    }
}
