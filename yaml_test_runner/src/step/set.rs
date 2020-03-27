use quote::{ToTokens, Tokens};

use super::Step;
use yaml_rust::Yaml;
use crate::step::BodyExpr;

pub struct Set {
    ident: syn::Ident,
    expr: String,
}

impl From<Set> for Step {
    fn from(set: Set) -> Self {
        Step::Set(set)
    }
}

impl BodyExpr for Set {}

impl Set {
    pub fn try_parse(yaml: &Yaml) -> Result<Set, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (expr, id) = {
            let (k, yaml) = hash.iter().next().unwrap();
            let key = k.as_str().ok_or_else(|| {
                failure::err_msg(format!("Expected string key but found {:?}", k))
            })?;

            let id = yaml.as_str().ok_or_else(|| {
                failure::err_msg(format!("Expected string value but found {:?}", k))
            })?;

            (key, id)
        };

        Ok(Set {
            ident: syn::Ident::from(id),
            expr: expr.into()
        })
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let ident = &self.ident;
        let expr = syn::Ident::from(self.body_expr(&self.expr));

        // TODO: Unwrap serde_json value here, or in the usage?
        tokens.append(quote!{
            let #ident = response_body#expr.clone();
        });
    }
}