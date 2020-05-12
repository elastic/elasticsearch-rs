use quote::{ToTokens, Tokens};

use super::Step;
use crate::step::Expr;
use yaml_rust::Yaml;

pub struct IsTrue {
    pub(crate) expr: Expr,
}

impl From<IsTrue> for Step {
    fn from(is_true: IsTrue) -> Self {
        Step::IsTrue(is_true)
    }
}

impl IsTrue {
    pub fn try_parse(yaml: &Yaml) -> Result<IsTrue, failure::Error> {
        let expr = yaml
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string key but found {:?}", &yaml)))?;

        Ok(IsTrue {
            expr: expr.into(),
        })
    }
}

impl ToTokens for IsTrue {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if self.expr.is_empty() {
            tokens.append(quote! {
                assert!(!text.is_empty());
            });
        } else {
            let expr = self.expr.expression();
            let ident = syn::Ident::from(expr.as_str());
            tokens.append(quote! {
                match &json#ident {
                    Value::Null => assert!(false, "Expected value at {} to be true but is null", #expr),
                    Value::Bool(b) => assert!(*b, "Expected value at {} to be true but is {}", #expr, b),
                    Value::Number(n) => assert!(n.as_f64().unwrap() != 0.0, "Expected value at {} to be true but is {}", #expr, n.as_f64().unwrap()),
                    Value::String(s) => assert!(!s.is_empty(), "Expected value at {} to be true but is {}", #expr, &s),
                    v => assert!(true, "Expected value at {} to be true but is {:?}", #expr, &v),
                }
            });
        }
    }
}
