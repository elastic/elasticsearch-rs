use quote::{ToTokens, Tokens};

use super::Step;
use crate::step::Expr;
use yaml_rust::Yaml;

pub struct IsFalse {
    pub(crate) expr: Expr,
}

impl From<IsFalse> for Step {
    fn from(is_false: IsFalse) -> Self {
        Step::IsFalse(is_false)
    }
}

impl IsFalse {
    pub fn try_parse(yaml: &Yaml) -> Result<IsFalse, failure::Error> {
        let expr = yaml.as_str().ok_or_else(|| {
            failure::err_msg(format!("expected string key but found {:?}", &yaml))
        })?;

        Ok(IsFalse { expr: expr.into() })
    }
}

impl ToTokens for IsFalse {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if self.expr.is_empty() {
            tokens.append(quote! {
                assert!(text.is_empty(), "expected value to be empty but was {}", &text);
            });
        } else {
            let expr = self.expr.expression();
            let ident = syn::Ident::from(expr.as_str());
            tokens.append(quote! {
                assert_is_false!(&json#ident);
            });
        }
    }
}
