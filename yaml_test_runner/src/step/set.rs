use quote::{ToTokens, Tokens};

use super::Step;
use crate::step::Expr;
use yaml_rust::Yaml;

pub struct Set {
    ident: syn::Ident,
    expr: Expr,
}

impl From<Set> for Step {
    fn from(set: Set) -> Self {
        Step::Set(set)
    }
}

impl Set {
    pub fn try_parse(yaml: &Yaml) -> Result<Set, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string key but found {:?}", k)))?;

        let id = v
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string value but found {:?}", v)))?;

        Ok(Set {
            ident: syn::Ident::from(id),
            expr: expr.into(),
        })
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let ident = &self.ident;
        let expr = syn::Ident::from(self.expr.expression().as_str());
        tokens.append(quote! {
            let #ident = response_body#expr.clone();
        });
    }
}
