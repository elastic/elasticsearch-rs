use super::Step;
use crate::step::{clean_regex, Expr};
use quote::{ToTokens, Tokens};
use yaml_rust::Yaml;

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
        Ok(Match { expr: expr.into(), value: v.clone() })
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
                        let ident = syn::Ident::from(expr.as_str());
                        tokens.append(quote! {
                            let regex = regex::RegexBuilder::new(#s)
                                .ignore_whitespace(true)
                                .build()?;
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
                    let ident = syn::Ident::from(expr.as_str());

                    // handle set values
                    let t = if s.starts_with('$') {
                        let t = s
                            .trim_start_matches('$')
                            .trim_start_matches('{')
                            .trim_end_matches('}');
                        let ident = syn::Ident::from(t);
                        quote! { #ident.as_str().unwrap() }
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
                if self.expr.is_body() {
                    panic!("match on $body with integer");
                } else {
                    let ident = syn::Ident::from(expr.as_str());
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
