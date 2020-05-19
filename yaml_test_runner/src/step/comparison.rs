use quote::{ToTokens, Tokens};

use super::Step;
use crate::step::Expr;
use yaml_rust::Yaml;

pub const OPERATORS: [&str; 4] = ["lt", "lte", "gt", "gte"];

pub struct Comparison {
    pub(crate) expr: Expr,
    value: Yaml,
    op: String,
}

impl From<Comparison> for Step {
    fn from(comparison: Comparison) -> Self {
        Step::Comparison(comparison)
    }
}

impl Comparison {
    pub fn try_parse(yaml: &Yaml, op: &str) -> Result<Comparison, failure::Error> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| failure::err_msg(format!("expected hash but found {:?}", yaml)))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k
            .as_str()
            .ok_or_else(|| failure::err_msg(format!("expected string key but found {:?}", k)))?;

        Ok(Comparison {
            expr: expr.into(),
            value: v.clone(),
            op: op.into(),
        })
    }

    fn assert<T: PartialOrd + ToTokens>(&self, t: T, expr: &str, op: &str, tokens: &mut Tokens) {
        let ident = syn::Ident::from(expr);
        let op_ident = syn::Ident::from(op);
        let message = "Expected value at {} to be numeric but is {}";
        let comparison_message = "Expected value at {} to be {:?} {}, but is {}";
        tokens.append(quote! {
            match &json#ident {
                Value::Number(n) => {
                    match n.as_i64() {
                        Some(i) => assert!(i #op_ident #t as i64, #comparison_message, #expr, #op, #t, i),
                        None => match n.as_f64() {
                            Some(f) => assert!(f #op_ident #t as f64, #comparison_message, #expr, #op, #t, f),
                            None => match n.as_u64() {
                                Some(u) => assert!(u #op_ident #t as u64, #comparison_message, #expr, #op, #t, u),
                                None => assert!(false, #message, #expr, &n)
                            }
                        }
                    }
                }
                v => assert!(false, #message, #expr, &v),
            }
        });
    }
}

impl ToTokens for Comparison {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let expr = self.expr.expression();
        let op = match self.op.as_str() {
            "lte" => "<=",
            "lt" => "<",
            "gt" => ">",
            "gte" => ">=",
            n => panic!("unsupported op {}", n),
        };

        match self.value.as_i64() {
            Some(i) => self.assert(i, &expr, op, tokens),
            None => match self.value.as_f64() {
                Some(f) => self.assert(f, &expr, op, tokens),
                None => {
                    match self.value.as_str() {
                        // handle "set" values
                        Some(s) if s.starts_with('$') => {
                            let s = s
                                .trim_start_matches('$')
                                .trim_start_matches('{')
                                .trim_end_matches('}');
                            let expr_ident = syn::Ident::from(expr.as_str());
                            let ident = syn::Ident::from(s);
                            let op_ident = syn::Ident::from(op);
                            let message = "Expected value at {} to be numeric but was {}";
                            let comparison_message =
                                "Expected value at {} to be {:?} {}, but was {}";
                            tokens.append(quote! {
                                match &json#expr_ident {
                                    Value::Number(n) => {
                                        match n.as_i64() {
                                            Some(i) => assert!(i #op_ident #ident.as_i64().unwrap(), #comparison_message, #expr, #op, #ident.as_i64().unwrap(), i),
                                            None => match n.as_f64() {
                                                Some(f) => assert!(f #op_ident #ident.as_f64().unwrap(), #comparison_message, #expr, #op, #ident.as_f64().unwrap(), f),
                                                None => match n.as_u64() {
                                                    Some(u) => assert!(u #op_ident #ident.as_u64().unwrap(), #comparison_message, #expr, #op, #ident.as_u64().unwrap(), u),
                                                    None => assert!(false, #message, #expr, &n)
                                                }
                                            }
                                        }
                                    }
                                    v => assert!(false, #message, #expr, &v),
                                }
                            });
                        }
                        _ => panic!("Expected i64 or f64 but found {:?}", &self.value),
                    }
                }
            },
        }
    }
}
