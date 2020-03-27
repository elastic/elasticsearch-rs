use quote::{ToTokens, Tokens};

use super::Step;
use yaml_rust::Yaml;
use crate::step::BodyExpr;

pub struct Length {
    len: usize,
    pub(crate) expr: String,
}

impl From<Length> for Step {
    fn from(length: Length) -> Self {
        Step::Length(length)
    }
}

impl BodyExpr for Length {}

impl Length {
    pub fn try_parse(yaml: &Yaml) -> Result<Length, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (expr, len) = {
            let (k, v) = hash.iter().next().unwrap();
            let key = k.as_str().ok_or_else(|| {
                failure::err_msg(format!("expected string key but found {:?}", k))
            })?;

            let len = v.as_i64().ok_or_else(|| {
                failure::err_msg(format!("expected i64 but found {:?}", v))
            })?;

            (key, len)
        };

        Ok(Length {
            len: len as usize,
            expr: expr.into()
        })
    }
}

impl ToTokens for Length {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let len = self.len;
        let expr = self.body_expr(&self.expr);

        if self.is_body_expr(&expr) {
            tokens.append(quote!{
                let len = string_response_body.len();
                assert_eq!(#len, len);
            });
        } else {
            let ident = syn::Ident::from(expr);
            tokens.append(quote!{
                let len = util::len_from_value(&response_body#ident)?;
                assert_eq!(#len, len);
            });
        }
    }
}