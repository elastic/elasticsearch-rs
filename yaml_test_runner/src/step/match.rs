use super::Step;
use quote::{ToTokens, Tokens};
use yaml_rust::Yaml;
use crate::step::BodyExpr;

pub struct Match {
    pub expr: String,
    value: Yaml,
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

        let (expr, value) = {
            let (k, v) = hash.iter().next().unwrap();
            let key = k.as_str().unwrap().trim().to_string();
            (key, v.clone())
        };

        Ok(Match { expr, value })
    }
}

impl ToTokens for Match {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let expr = self.body_expr(&self.expr);

        match &self.value {
            Yaml::String(s) => {
                if s.starts_with('/') {
                    // trim the enclosing forward slashes and replace escaped forward slashes
                    let s = s.trim().trim_matches('/').replace("\\/", "/");
                    if self.is_body_expr(&expr) {
                        tokens.append(quote! {
                            let regex = regex::RegexBuilder::new(#s)
                                .ignore_whitespace(true)
                                .build()?;
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
                if self.is_body_expr(&expr) {
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
