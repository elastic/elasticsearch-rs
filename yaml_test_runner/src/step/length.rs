use quote::{ToTokens, Tokens};

use super::Step;
use crate::step::Expr;
use yaml_rust::Yaml;

pub struct Length {
    pub(crate) expr: Expr,
    len: usize,
}

impl From<Length> for Step {
    fn from(length: Length) -> Self {
        Step::Length(length)
    }
}

impl Length {
    pub fn try_parse(yaml: &Yaml) -> Result<Length, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();

        let expr = k
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string key but found {:?}", k)))?;

        let len = v
            .as_i64()
            .ok_or_else(|| failure::err_msg(format!("expected i64 but found {:?}", v)))?;

        Ok(Length {
            len: len as usize,
            expr: expr.into(),
        })
    }
}

impl ToTokens for Length {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let len = self.len;

        if self.expr.is_body() {
            tokens.append(quote! {
                let len = util::len_from_value(&response_body)?;
                assert_eq!(#len, len);
            });
        } else {
            let expr = self.expr.expression();
            let ident = syn::Ident::from(expr);
            tokens.append(quote! {
                let len = util::len_from_value(&response_body#ident)?;
                assert_eq!(#len, len);
            });
        }
    }
}
