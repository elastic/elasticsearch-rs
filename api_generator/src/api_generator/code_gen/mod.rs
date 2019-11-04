pub mod enums;
pub mod namespace_clients;
pub mod root;
pub mod url;

use crate::api_generator::{ApiEndpoint, HttpMethod, Type, TypeKind};
use inflector::Inflector;
use quote::{ToTokens, Tokens};
use std::collections::BTreeMap;
use std::str;
use syn::{Field, FieldValue, FnArg, ImplItem};
use url::url_builder::{IntoExpr, Path, UrlBuilder};

/// AST for a literal
fn lit<I: Into<String>>(lit: I) -> syn::Lit {
    syn::Lit::Str(lit.into(), syn::StrStyle::Cooked)
}

/// AST for an identifier
fn ident<I: AsRef<str>>(name: I) -> syn::Ident {
    syn::Ident::from(name.as_ref())
}

/// AST for doc attribute
fn doc(comment: String) -> syn::Attribute {
    syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::NameValue(ident("doc".to_string()), lit(comment)),
        is_sugared_doc: true,
    }
}

/// AST for an expression parsed from quoted tokens
pub fn parse_expr(input: quote::Tokens) -> syn::Expr {
    syn::parse_expr(input.to_string().as_ref()).unwrap()
}

/// Ensures that the name generated is one that is valid for Rust
fn valid_name(s: &str) -> &str {
    match s {
        "type" => "ty",
        s => s,
    }
}

/// AST for a simple path variable.
fn path_none(path_ident: &str) -> syn::Path {
    path(path_ident, vec![], vec![])
}

/// AST for a path variable.
fn path(path: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Ty>) -> syn::Path {
    path_segments(vec![(path, lifetimes, types)])
}

/// AST for a path variable.
fn path_segments(paths: Vec<(&str, Vec<syn::Lifetime>, Vec<syn::Ty>)>) -> syn::Path {
    syn::Path {
        global: false,
        segments: paths
            .into_iter()
            .map(|(path, lifetimes, types)| syn::PathSegment {
                ident: syn::Ident::new(valid_name(path)),
                parameters: syn::PathParameters::AngleBracketed(syn::AngleBracketedParameterData {
                    lifetimes,
                    types,
                    bindings: vec![],
                }),
            })
            .collect(),
    }
}

pub trait GetPath {
    fn get_path(&self) -> &syn::Path;
}

impl GetPath for syn::Ty {
    fn get_path(&self) -> &syn::Path {
        match *self {
            syn::Ty::Path(_, ref p) => &p,
            _ => panic!("Only path types are supported."),
        }
    }
}

impl GetPath for syn::Path {
    fn get_path(&self) -> &syn::Path {
        &self
    }
}

/// Gets the Ty syntax token for a TypeKind
fn ty(name: &str, kind: &TypeKind, required: bool) -> syn::Ty {
    let mut v = String::new();
    if !required {
        v.push_str("Option<");
    }

    match kind {
        TypeKind::None => v.push_str("String"),
        TypeKind::List => v.push_str("Vec<String>"),
        TypeKind::Enum => v.push_str(name.to_pascal_case().as_str()),
        TypeKind::String => v.push_str("String"),
        TypeKind::Text => v.push_str("String"),
        TypeKind::Boolean => v.push_str("bool"),
        TypeKind::Number => v.push_str("i64"),
        TypeKind::Float => v.push_str("f32"),
        TypeKind::Double => v.push_str("f64"),
        TypeKind::Integer => v.push_str("i32"),
        TypeKind::Long => v.push_str("i64"),
        TypeKind::Date => v.push_str("String"),
        TypeKind::Time => v.push_str("String"),
    };

    if !required {
        v.push_str(">");
    }

    syn::parse_type(v.as_str()).unwrap()
}

/// A standard `'a` lifetime
pub fn lifetime_a() -> syn::Lifetime {
    syn::Lifetime {
        ident: syn::Ident::new("'a"),
    }
}

pub trait HasLifetime {
    fn has_lifetime(&self) -> bool;
}

impl<T: GetPath> HasLifetime for T {
    fn has_lifetime(&self) -> bool {
        match self.get_path().segments[0].parameters {
            syn::PathParameters::AngleBracketed(ref params) => !params.lifetimes.is_empty(),
            _ => false,
        }
    }
}

/// Generics with a standard `'a` lifetime
pub fn generics_a() -> syn::Generics {
    generics(vec![lifetime_a()], vec![])
}

/// Generics with no parameters.
pub fn generics_none() -> syn::Generics {
    generics(vec![], vec![])
}

/// Generics with the given lifetimes and type bounds.
pub fn generics(lifetimes: Vec<syn::Lifetime>, types: Vec<syn::TyParam>) -> syn::Generics {
    syn::Generics {
        lifetimes: lifetimes
            .into_iter()
            .map(|l| syn::LifetimeDef {
                attrs: vec![],
                lifetime: l,
                bounds: vec![],
            })
            .collect(),
        ty_params: types,
        where_clause: syn::WhereClause::none(),
    }
}

fn create_required_field(f: (&String, &Type)) -> syn::Field {
    create_field(f, true)
}

fn create_optional_field(f: (&String, &Type)) -> syn::Field {
    create_field(f, false)
}

/// Creates a field for a struct
fn create_field(f: (&String, &Type), required: bool) -> syn::Field {
    syn::Field {
        ident: Some(ident(valid_name(&f.0).to_lowercase())),
        vis: syn::Visibility::Inherited,
        attrs: vec![],
        ty: ty(&f.0, &f.1.ty, required),
    }
}

fn create_required_fn(f: (&String, &Type)) -> syn::ImplItem {
    create_fn(f, true)
}

fn create_optional_fn(f: (&String, &Type)) -> syn::ImplItem {
    create_fn(f, false)
}

/// Creates a builder fn for a builder impl
fn create_fn(f: (&String, &Type), required: bool) -> syn::ImplItem {
    let name = valid_name(&f.0).to_lowercase();
    let impl_ident = ident(&name);
    let field_ident = ident(&name);
    let value_ident = ident(&name);
    let ty = ty(&f.0, &f.1.ty, required);
    let doc_attr = match &f.1.description {
        Some(docs) => vec![doc(docs.into())],
        _ => vec![],
    };

    syn::ImplItem {
        ident: impl_ident,
        vis: syn::Visibility::Public,
        defaultness: syn::Defaultness::Final,
        attrs: doc_attr,
        node: syn::ImplItemKind::Method(
            syn::MethodSig {
                unsafety: syn::Unsafety::Normal,
                constness: syn::Constness::NotConst,
                abi: None,
                decl: syn::FnDecl {
                    inputs: vec![
                        syn::FnArg::SelfValue(syn::Mutability::Mutable),
                        syn::FnArg::Captured(syn::Pat::Path(None, path_none(name.as_str())), ty),
                    ],
                    // TODO: is there a Self syn type?
                    output: syn::FunctionRetTy::Ty(syn::parse_type("Self").unwrap()),
                    variadic: false,
                },
                generics: generics_none(),
            },
            // generates a fn body of the form
            // --------
            // self.<field> = <field>;
            // self
            // ---------
            syn::Block {
                stmts: vec![
                    syn::Stmt::Semi(Box::new(parse_expr(
                        quote!(self.#field_ident = #value_ident),
                    ))),
                    syn::Stmt::Expr(Box::new(parse_expr(quote!(self)))),
                ],
            },
        ),
    }
}

/// use declarations common across builders
pub fn use_declarations() -> Tokens {
    quote!(
        use crate::{
            client::{Elasticsearch, Sender},
            enums::*,
            error::ElasticsearchError,
            http_method::HttpMethod,
            response::ElasticsearchResponse,
        };
        use reqwest::{Response, Request, Error, StatusCode, header::HeaderMap};
        use serde::{Serialize, de::DeserializeOwned};
    )
}

/// Creates the function arguments for a builder struct new fn
fn create_new_fn_args(required_parts: &[(&String, &Type)]) -> Vec<FnArg> {
    match required_parts.len() {
        0 => vec![],
        _ => required_parts
            .iter()
            .map(|part| (valid_name(part.0), part.1))
            .map(|part| {
                syn::FnArg::Captured(
                    syn::Pat::Path(None, path_none(part.0)),
                    ty(part.0, &part.1.ty, true),
                )
            })
            .collect::<Vec<syn::FnArg>>(),
    }
}

/// Creates the field values for a builder struct new fn call
fn create_new_fields(required_parts: &[(&String, &Type)]) -> Vec<FieldValue> {
    match required_parts.len() {
        0 => vec![],
        _ => required_parts
            .iter()
            .map(|part| valid_name(part.0))
            .map(|part| syn::FieldValue {
                attrs: vec![],
                ident: ident(part),
                expr: syn::ExprKind::Path(None, path_none(ident(part).as_ref())).into(),
                is_shorthand: false,
            })
            .collect(),
    }
}

/// Creates the AST for field values initialized with a default value.
///
/// Since all default values are Option<T>, the default value for all is None
fn create_default_fields(default_fields: &[&syn::Ident]) -> Vec<FieldValue> {
    default_fields
        .iter()
        .map(|part| syn::FieldValue {
            attrs: vec![],
            ident: ident(part),
            expr: syn::ExprKind::Path(None, path_none(ident("None").as_ref())).into(),
            is_shorthand: false,
        })
        .collect()
}

/// Creates the AST for a new fn for a builder struct
fn create_new_fn(
    builder_ident: &syn::Ident,
    required_parts: &[(&String, &Type)],
    default_fields: &[&syn::Ident],
) -> Tokens {
    let fn_args = create_new_fn_args(&required_parts);
    let fields = create_new_fields(&required_parts);
    let default_fields = create_default_fields(default_fields);
    match required_parts.len() {
        0 => quote!(
            pub fn new(client: Elasticsearch) -> Self {
                #builder_ident {
                    client,
                    #(#default_fields),*,
                }
            }
        ),
        _ => quote!(
             pub fn new(client: Elasticsearch, #(#fn_args),*) -> Self {
                #builder_ident {
                    client,
                    #(#fields),*,
                    #(#default_fields),*,
                }
            }
        ),
    }
}

/// creates the AST for a builder struct
pub fn create_builder_struct(
    builder_name: String,
    endpoint: &ApiEndpoint,
    common_fields: &[Field],
    common_builder_fns: &[ImplItem],
    common_params: &BTreeMap<String, Type>,
) -> Tokens {
    let supports_body = endpoint.supports_body();
    let builder_ident = ident(&builder_name);

    // url parts that are common across all urls.
    // These are required parameters for the builder ctor new() fn
    let required_parts = endpoint.url.required_part_names();

    // collect all the fields for the builder struct. Start with url parts
    let mut fields: Vec<syn::Field> = endpoint
        .url
        .parts
        .iter()
        .map(|f| create_field(f, required_parts.contains(&&**f.0)))
        .collect();

    if supports_body {
        fields.push(syn::Field {
            ident: Some(ident("body")),
            vis: syn::Visibility::Inherited,
            attrs: vec![],
            ty: syn::parse_type("Option<B>").unwrap(),
        })
    }

    // url parameters
    fields.append(
        &mut endpoint
            .url
            .params
            .iter()
            .map(create_optional_field)
            .collect(),
    );

    // Combine common fields with struct fields, sort and deduplicate
    // clone common_fields, since quote!() consumes the Vec<Field>
    fields.append(&mut common_fields.to_vec().clone());
    fields.sort_by(|a, b| a.ident.cmp(&b.ident));
    fields.dedup_by(|a, b| a.ident.eq(&b.ident));

    // collect all the functions for the builder struct
    let mut builder_fns: Vec<ImplItem> = endpoint
        .url
        .params
        .iter()
        .map(|f| create_fn(f, required_parts.contains(&&**f.0)))
        .collect();

    builder_fns.append(
        &mut endpoint
            .url
            .parts
            .iter()
            .map(|f| create_fn(f, required_parts.contains(&&**f.0)))
            .collect(),
    );

    if supports_body {
        builder_fns.push(syn::ImplItem {
            ident: ident("body"),
            vis: syn::Visibility::Public,
            defaultness: syn::Defaultness::Final,
            attrs: vec![doc("The body for the API call".into())],
            node: syn::ImplItemKind::Method(
                syn::MethodSig {
                    unsafety: syn::Unsafety::Normal,
                    constness: syn::Constness::NotConst,
                    abi: None,
                    decl: syn::FnDecl {
                        inputs: vec![
                            syn::FnArg::SelfValue(syn::Mutability::Mutable),
                            syn::FnArg::Captured(
                                syn::Pat::Path(None, path_none("body")),
                                syn::parse_type("Option<B>").unwrap(),
                            ),
                        ],
                        // TODO: is there a Self syn type?
                        output: syn::FunctionRetTy::Ty(syn::parse_type("Self").unwrap()),
                        variadic: false,
                    },
                    generics: generics_none(),
                },
                // generates a fn body of the form
                // --------
                // self.<field> = <field>;
                // self
                // ---------
                syn::Block {
                    stmts: vec![
                        syn::Stmt::Semi(Box::new(parse_expr(quote!(self.body = body)))),
                        syn::Stmt::Expr(Box::new(parse_expr(quote!(self)))),
                    ],
                },
            ),
        });
    }

    // Combine common fns with builder fns, sort and deduplicate
    // clone is required, since quote!() consumes the Vec<Item>
    builder_fns.append(&mut common_builder_fns.to_vec().clone());
    builder_fns.sort_by(|a, b| a.ident.cmp(&b.ident));
    builder_fns.dedup_by(|a, b| a.ident.eq(&b.ident));

    let default_fields = {
        let r: Vec<&str> = required_parts.iter().map(|p| valid_name(p)).collect();

        fields
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .filter(|f| !r.contains(&valid_name(f.to_string().as_str())))
            .collect::<Vec<_>>()
    };

    let new_fn = create_new_fn(
        &builder_ident,
        &endpoint.url.required_parts(),
        &default_fields,
    );

    let path_expr = create_path_expression(&endpoint);
    let method = create_method_expression(&builder_name, &endpoint);
    let query_string_params = {
        let mut p = endpoint.url.params.clone();
        p.append(&mut common_params.clone());
        p
    };

    let query_string_expr = create_query_string_expression(&query_string_params);

    let body_expr = {
        if supports_body {
            quote!(self.body)
        } else {
            quote!(Option::<()>::None)
        }
    };

    let (builder_expr, builder_impl, sender_impl) = {
        if supports_body {
            let builder_expr = quote!(#builder_ident<B>);
            (
                quote!(#builder_expr),
                quote!(impl<B> #builder_expr where B: Serialize),
                quote!(impl<B> Sender for #builder_expr where B: Serialize),
            )
        } else {
            (
                quote!(#builder_ident),
                quote!(impl #builder_ident),
                quote!(impl Sender for #builder_ident),
            )
        }
    };

    quote!(
        pub struct #builder_expr {
            client: Elasticsearch,
            #(#fields),*,
        }

        #builder_impl {
            #new_fn
            #(#builder_fns)*
        }

        #sender_impl {
            fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
                  let path = #path_expr;
                  let method = #method;
                  let query_string = #query_string_expr;
                  let body = #body_expr;
                  let response = self.client.send(method, &path, query_string.as_ref(), body)?;
                  Ok(response)
            }
        }
    )
}

/// Creates the AST for a fn that returns a new instance of a builder struct
pub fn create_builder_struct_ctor_fns(
    builder_name: String,
    name: &str,
    endpoint: &ApiEndpoint,
    is_root_method: bool,
) -> Tokens {
    let builder_ident = ident(builder_name.to_pascal_case());

    let (fn_name, builder_ident_ret) = {
        let i = ident(name.to_string());
        let b = ident(builder_name.to_pascal_case());
        if endpoint.supports_body() {
            (quote!(#i<B>), quote!(#b<B> where B: Serialize))
        } else {
            (quote!(#i), quote!(#b))
        }
    };

    let method_doc = match &endpoint.documentation {
        Some(docs) => Some(doc(docs.into())),
        _ => None,
    };

    let fn_args = create_new_fn_args(&endpoint.url.required_parts());
    let builder_args: Vec<syn::Pat> = fn_args
        .clone()
        .into_iter()
        .filter_map(|f| match f {
            FnArg::Captured(p, _) => Some(p),
            _ => None,
        })
        .collect();

    let clone_expr = {
        if is_root_method {
            quote!(self.clone())
        } else {
            quote!(self.client.clone())
        }
    };

    quote!(
        #method_doc
        pub fn #fn_name(&self, #(#fn_args),*) -> #builder_ident_ret {
            #builder_ident::new(#clone_expr,#(#builder_args),*)
        }
    )
}

fn create_self_path_pat(name: &str) -> syn::Pat {
    let mut s = String::from("&self.");
    s.push_str(valid_name(name));
    syn::Pat::Path(None, path_none(&s))
}

fn create_match_arm_pat(path_url_params: &[&str], name: &str) -> syn::Pat {
    if path_url_params.contains(&name) {
        create_some_match_arm(name)
    } else {
        create_none_match_arm()
    }
}

fn create_some_match_arm(name: &str) -> syn::Pat {
    let ident = ident(valid_name(name));
    let t = syn::Pat::Ident(
        syn::BindingMode::ByValue(syn::Mutability::Immutable),
        ident,
        None,
    );
    syn::Pat::TupleStruct(path_none("Some"), vec![t], None)
}

fn create_none_match_arm() -> syn::Pat {
    syn::Pat::Path(None, path_none("None"))
}

/// Creates the AST for the expression to select the correct API url path, based on inputs
pub fn create_path_expression(endpoint: &ApiEndpoint) -> syn::Expr {
    let required_parts = endpoint.url.required_part_names();
    let (required_map, optional_map): (BTreeMap<_,_>, BTreeMap<_,_>) = endpoint.url.parts
        .iter()
        .partition(|&(p, _)| required_parts.contains(&p.as_str()));

    match endpoint.url.all_paths().len() {
        1 => {
            let single_path = *endpoint.url.all_paths().iter().next().unwrap();
            url::url_builder::UrlBuilder::new(single_path.split(), &required_map, &optional_map)
                .build()
        }
        _ => {
            // generates the <expr> part in a "match <expr>"
            let optional_parts = endpoint.url.optional_parts();
            let match_expr = match optional_parts.len() {
                1 => {
                    let (name, _) = optional_parts[0];
                    create_self_path_pat(name)
                }
                _ => {
                    let pats: Vec<syn::Pat> = optional_parts
                        .iter()
                        .map(|&(name, _)| create_self_path_pat(name))
                        .collect();
                    syn::Pat::Tuple(pats, None)
                }
            };

            let mut arms = vec![];
            for path in endpoint.url.all_paths() {
                // deprecated paths that duplicate all the _mapping paths
                if path.0.contains("_mappings") {
                    continue;
                }

                let path_url_params = path.params();
                let pat = match optional_parts.len() {
                    0 => create_none_match_arm(),
                    1 => {
                        // single value match arm expression
                        let (name, _) = optional_parts[0];
                        create_match_arm_pat(&path_url_params, name)
                    }
                    _ => {
                        // tuple match arm expression
                        let tuple_pats = optional_parts
                            .iter()
                            .map(|&(name, _)| create_match_arm_pat(&path_url_params, name))
                            .collect();
                        syn::Pat::Tuple(tuple_pats, None)
                    }
                };

                let body =
                    url::url_builder::UrlBuilder::new(path.split(), &required_map, &optional_map)
                        .build();
                arms.push(syn::Arm {
                    attrs: vec![],
                    pats: vec![pat],
                    guard: None,
                    body: Box::new(body),
                });

                // HACK: to add additional match arms for exhaustive matching.
                // TODO: These will go away with the move to enums
                if path.0.contains(&"{index}")
                    && path.0.contains(&"{type}")
                    && !path.0.contains(&"{index}/{type}/{id}")
                    && !path.0.contains(&"_mapping")
                    && !required_map.contains_key(&"index".to_string())
                {
                    let mut path_string = path.0.clone();
                    path_string = path_string.replace("{index}", "_all");

                    let sub_path = Path(path_string);
                    let sub_params = sub_path.params();
                    let tuple_pats = optional_parts
                        .iter()
                        .map(|&(name, _)| create_match_arm_pat(&sub_params, name))
                        .collect();

                    let pat = syn::Pat::Tuple(tuple_pats, None);
                    let body =
                        UrlBuilder::new(sub_path.split(), &required_map, &optional_map).build();

                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![pat],
                        guard: None,
                        body: Box::new(body),
                    });
                } else if path.0.contains(&"{metric}/{index}") {
                    let mut path_string_clone = path.0.clone();
                    path_string_clone = path_string_clone.replace("{metric}", "_all");
                    let path_clone = Path(path_string_clone);
                    let path_clone_params = path_clone.params();
                    let tuple_pats = optional_parts
                        .iter()
                        .map(|&(name, _)| create_match_arm_pat(&path_clone_params, name))
                        .collect();

                    let pat = syn::Pat::Tuple(tuple_pats, None);
                    let body =
                        UrlBuilder::new(path_clone.split(), &required_map, &optional_map).build();
                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![pat],
                        guard: None,
                        body: Box::new(body),
                    });
                } else if path.0 == "/_nodes/stats/{metric}" {
                    let mut path_string_clone = path.0.clone();
                    path_string_clone = path_string_clone
                        .replace("stats/{metric}", "{node_id}/stats/all/{index_metric}");
                    let path_clone = Path(path_string_clone);
                    let body =
                        UrlBuilder::new(path_clone.split(), &required_map, &optional_map).build();
                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![syn::Pat::Tuple(
                            vec![
                                create_some_match_arm("index_metric"),
                                create_none_match_arm(),
                                create_some_match_arm("node_id"),
                            ],
                            None,
                        )],
                        guard: None,
                        body: Box::new(body),
                    });

                    let mut path_string_clone = path.0.clone();
                    path_string_clone = path_string_clone.replace("{metric}", "all/{index_metric}");
                    let path_clone = Path(path_string_clone);
                    let body =
                        UrlBuilder::new(path_clone.split(), &required_map, &optional_map).build();
                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![syn::Pat::Tuple(
                            vec![
                                create_some_match_arm("index_metric"),
                                create_none_match_arm(),
                                create_none_match_arm(),
                            ],
                            None,
                        )],
                        guard: None,
                        body: Box::new(body),
                    });
                } else if path.0 == "/_snapshot/{repository}/_status"
                    || path.0 == "/_security/privilege/{application}"
                {
                    let message = lit(format!("{} must also be specified", path_url_params[0]));
                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![syn::Pat::Tuple(
                            vec![create_none_match_arm(), create_some_match_arm("_")],
                            None,
                        )],
                        guard: None,
                        body: Box::new(parse_expr(quote!(panic!(#message)))),
                    });
                } else if path.0 == "/_mapping/{type}" && endpoint.methods != vec![HttpMethod::Get]
                {
                    arms.push(syn::Arm {
                        attrs: vec![],
                        pats: vec![syn::Pat::Tuple(
                            vec![create_none_match_arm(), create_none_match_arm()],
                            None,
                        )],
                        guard: None,
                        body: Box::new(parse_expr(quote!(std::borrow::Cow::Borrowed(
                            "/_all/_mapping"
                        )))),
                    });
                }
            }

            // build the complete match expression
            syn::ExprKind::Match(Box::new(match_expr.into_expr()), arms).into()
        }
    }
}

/// Create the AST for an expression that assigns a HttpMethod value
pub fn create_method_expression(builder_name: &str, endpoint: &ApiEndpoint) -> syn::Expr {
    match endpoint.methods.len() {
        1 => {
            let method = endpoint.methods.first().unwrap();
            let mut tokens = Tokens::new();
            method.to_tokens(&mut tokens);
            parse_expr(tokens)
        }
        _ => match endpoint.methods.as_slice() {
            [HttpMethod::Post, HttpMethod::Put] => {
                if builder_name.contains("Put") {
                    parse_expr(quote!(HttpMethod::Put))
                } else {
                    parse_expr(quote!(HttpMethod::Post))
                }
            }
            [HttpMethod::Get, HttpMethod::Post] => parse_expr(quote!(match self.body {
                Some(_) => HttpMethod::Post,
                None => HttpMethod::Get,
            })),
            _ => panic!("Combination of methods unexpected"),
        },
    }
}

/// Create the AST for an expression that builds a struct of query string parameters
fn create_query_string_expression(endpoint_params: &BTreeMap<String, Type>) -> Tokens {
    if endpoint_params.is_empty() {
        quote!(None::<()>)
    } else {
        let query_struct_typ = ident("QueryParamsStruct");
        let struct_fields = endpoint_params.iter().map(|(param_name, param_type)| {
            let field = create_optional_field((param_name, param_type));
            let field_rename = lit(param_name);
            quote! {
                #[serde(rename = #field_rename)]
                #field
            }
        });
        let query_ctor = endpoint_params.iter().map(|(param_name, _)| {
            let field_name = ident(valid_name(param_name).to_lowercase());
            quote! {
                #field_name: self.#field_name
            }
        });
        quote! {
            {
                #[derive(Serialize)]
                struct #query_struct_typ {
                    #(#struct_fields,)*
                }
                let query_params = #query_struct_typ {
                    #(#query_ctor,)*
                };
                Some(query_params)
            }
        }
    }
}

pub fn shift_while<F>(i: &[u8], f: F) -> &[u8]
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;
    for c in i {
        if f(*c) {
            ctr += 1;
        } else {
            break;
        }
    }

    &i[ctr..]
}

pub fn take_while<F>(i: &[u8], f: F) -> (&[u8], &str)
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;

    for c in i {
        if f(*c) {
            ctr += 1;
        } else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn take_while1<F>(i: &[u8], f: F) -> (&[u8], &str)
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;

    for c in i {
        if f(*c) || ctr == 0 {
            ctr += 1;
        } else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn shift(i: &[u8], c: usize) -> &[u8] {
    match c {
        c if c >= i.len() => &[],
        _ => &i[c..],
    }
}
