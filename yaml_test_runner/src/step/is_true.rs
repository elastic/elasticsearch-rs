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
            // for a HEAD request, the body is expected to be empty, so check the status code instead.
            tokens.append(quote! {
                match method {
                    Method::Head => assert!(status_code.is_success(), "expected successful response for HEAD request but was {}", status_code.as_u16()),
                    _ => assert!(!text.is_empty(), "expected value to be true (not empty) but was {}", &text),
                }
            });
        } else {
            let expr = self.expr.expression();
            let ident = syn::Ident::from(expr.as_str());
            tokens.append(quote! {
                match &json#ident {
                    Value::Null => assert!(false, "expected value at {} to be true (not null) but was null", #expr),
                    Value::Bool(b) => assert!(*b, "expected value at {} to be true but was false", #expr),
                    Value::Number(n) => assert_ne!(n.as_f64().unwrap(), 0.0, "expected value at {} to be true (not 0) but was {}", #expr, n.as_f64().unwrap()),
                    Value::String(s) => assert!(!s.is_empty(), "expected value at {} to be true (not empty) but was {}", #expr, &s),
                    v => {},
                }
            });
        }
    }
}
