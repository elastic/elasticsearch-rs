// Parts modified from https://github.com/elastic-rs/elastic
// Licensed under Apache 2.0: https://github.com/elastic-rs/elastic/blob/51298dd64278f34d2db911bd1a35eb757c336198/LICENSE-APACHE

use crate::api_generator::code_gen::url::url_builder::{IntoExpr, UrlBuilder};
use crate::api_generator::{code_gen::*, ApiEndpoint, Path};
use inflector::Inflector;
use syn;

/// Builder for request url parts enum
///
/// The output of this structure is an enum that only accepts valid parameter combinations,
/// based on what's given in the paths for an endpoint.
#[derive(Debug, Clone)]
pub struct EnumBuilder<'a> {
    ident: syn::Ident,
    api_name: String,
    variants: Vec<syn::Variant>,
    paths: Vec<&'a Path>,
    has_lifetime: bool,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(prefix: &str) -> Self {
        let name = Self::name(prefix);
        let api_name = split_on_pascal_case(prefix);
        EnumBuilder {
            ident: ident(name),
            api_name,
            variants: vec![],
            paths: vec![],
            has_lifetime: false,
        }
    }

    fn name(prefix: &str) -> String {
        format!("{}UrlParts", prefix.to_pascal_case())
    }

    /// Whether this instance already contains a path with parts matching the given path
    fn contains_path_with_parts(&self, path: &'a Path) -> bool {
        let params = path.path.params();
        self.paths.iter().any(|&p| p.path.params() == params)
    }

    /// Whether this instance contains only a single path with no parts
    pub fn contains_single_parameterless_part(&self) -> bool {
        match self.paths.len() {
            1 => self.paths[0].parts.is_empty(),
            _ => false,
        }
    }

    pub fn with_path(mut self, path: &'a Path) -> Self {
        if !self.contains_path_with_parts(path) {
            let variant = match &path.parts.len() {
                0 => Self::parts_none(),
                _ => {
                    self.has_lifetime = true;
                    let name = &path
                        .path
                        .params()
                        .iter()
                        .map(|k| k.to_pascal_case())
                        .collect::<Vec<_>>()
                        .join("");
                    Self::parts(&name, &path)
                }
            };

            self.variants.push(variant);
            self.paths.push(path);
        }

        self
    }

    /// AST for a parts variant.
    fn parts(name: &str, path: &Path) -> syn::Variant {
        syn::Variant {
            ident: ident(name),
            attrs: vec![],
            discriminant: None,
            data: syn::VariantData::Tuple(
                path.path
                    .params()
                    .iter()
                    .map(|&p| {
                        let ty = path.parts[p].ty;
                        syn::Field {
                            ident: None,
                            vis: syn::Visibility::Inherited,
                            attrs: vec![],
                            ty: typekind_to_ty(p, ty, true),
                        }
                    })
                    .collect(),
            ),
        }
    }

    /// AST for a `None` parts variant.
    fn parts_none() -> syn::Variant {
        syn::Variant {
            ident: ident("None"),
            attrs: vec![],
            data: syn::VariantData::Unit,
            discriminant: None,
        }
    }

    fn match_path(ty: &syn::Ty, variant: &syn::Variant) -> syn::Path {
        let mut path = ty.get_path().to_owned();
        // Remove lifetimes from the enum type.
        for segment in &mut path.segments {
            segment.parameters = syn::PathParameters::none();
        }

        path.segments
            .push(syn::PathSegment::from(variant.ident.to_string()));
        path
    }

    /// Get the field names for the enum tuple variant to match.
    fn match_fields(path: &Path) -> Vec<syn::Pat> {
        path.path
            .params()
            .iter()
            .map(|&p| {
                syn::Pat::Ident(
                    syn::BindingMode::ByRef(syn::Mutability::Immutable),
                    ident(valid_name(p)),
                    None,
                )
            })
            .collect()
    }

    /// Build this enum and return ASTs for its type, struct declaration and impl
    pub fn build(self) -> (syn::Ty, syn::Item, syn::Item) {
        let variants = match self.variants.len() {
            0 => vec![Self::parts_none()],
            _ => self.variants,
        };

        let (enum_ty, generics) = {
            if self.has_lifetime {
                (ty_a(self.ident.as_ref()), generics_a())
            } else {
                (ty(self.ident.as_ref()), generics_none())
            }
        };

        let enum_impl = {
            let mut arms = Vec::new();
            for (variant, &path) in variants.iter().zip(self.paths.iter()) {
                let match_path = Self::match_path(&enum_ty, variant);
                let fields = Self::match_fields(path);

                let arm = match fields.len() {
                    0 => syn::Pat::Path(None, match_path),
                    _ => syn::Pat::TupleStruct(match_path, fields, None),
                };

                let body = UrlBuilder::new(path).build();
                arms.push(syn::Arm {
                    attrs: vec![],
                    pats: vec![arm],
                    guard: None,
                    body: Box::new(body),
                });
            }
            let match_expr: syn::Expr =
                syn::ExprKind::Match(Box::new(path_none("self").into_expr()), arms).into();

            let fn_decl = syn::FnDecl {
                inputs: vec![syn::FnArg::SelfValue(syn::Mutability::Immutable)],
                output: syn::FunctionRetTy::Ty(ty("Cow<'static, str>")),
                variadic: false,
            };

            let item = syn::ImplItem {
                ident: ident("build"),
                vis: syn::Visibility::Public,
                defaultness: syn::Defaultness::Final,
                attrs: vec![],
                node: syn::ImplItemKind::Method(
                    syn::MethodSig {
                        unsafety: syn::Unsafety::Normal,
                        constness: syn::Constness::NotConst,
                        abi: None,
                        decl: fn_decl,
                        generics: generics_none(),
                    },
                    syn::Block {
                        stmts: vec![match_expr.into_stmt()],
                    },
                ),
            };

            syn::Item {
                ident: ident(""),
                vis: syn::Visibility::Public,
                attrs: vec![],
                node: syn::ItemKind::Impl(
                    syn::Unsafety::Normal,
                    syn::ImplPolarity::Positive,
                    generics.clone(),
                    None,
                    Box::new(enum_ty.clone()),
                    vec![item],
                ),
            }
        };

        let enum_decl = syn::Item {
            ident: self.ident,
            vis: syn::Visibility::Public,
            attrs: vec![
                syn::Attribute {
                    is_sugared_doc: false,
                    style: syn::AttrStyle::Outer,
                    value: syn::MetaItem::List(
                        ident("derive"),
                        vec![
                            syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("Debug"))),
                            syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("Clone"))),
                            syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ident("PartialEq"))),
                        ],
                    ),
                },
                doc(format!("Url parts for the {} API", self.api_name)),
            ],
            node: syn::ItemKind::Enum(variants, generics),
        };

        (enum_ty, enum_decl, enum_impl)
    }
}

impl<'a> From<&'a (String, ApiEndpoint)> for EnumBuilder<'a> {
    fn from(value: &'a (String, ApiEndpoint)) -> Self {
        let endpoint = &value.1;
        let mut builder = EnumBuilder::new(value.0.to_pascal_case().as_ref());
        for path in &endpoint.url.paths {
            builder = builder.with_path(&path);
        }

        builder
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;
    use crate::api_generator::{Url, Path, HttpMethod, Body, Deprecated, Type, TypeKind, Documentation, ast_eq};
    use std::collections::BTreeMap;
    use crate::api_generator::code_gen::url::url_builder::PathString;

    #[test]
    fn generate_parts_enum_from_endpoint() {
        let endpoint = (
            "search".to_string(),
            ApiEndpoint {
                documentation: Documentation {
                    description: None,
                    url: None,
                },
                stability: "stable".to_string(),
                url: Url {
                    paths: vec![
                        Path {
                            path: PathString("/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: BTreeMap::new(),
                            deprecated: None,
                        },
                        Path {
                            path: PathString("/{index}/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: {
                                let mut map = BTreeMap::new();
                                map.insert("index".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of document types to search".to_string()),
                                    options: vec![],
                                    default: None,
                                });
                                map
                            },
                            deprecated: None,
                        },
                        Path {
                            path: PathString("/{index}/{type}/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: {
                                let mut map = BTreeMap::new();
                                map.insert("index".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of index names to search".to_string()),
                                    options: vec![],
                                    default: None,
                                });
                                map.insert("type".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of document types to search".to_string()),
                                    options: vec![],
                                    default: None,
                                });
                                map
                            },
                            deprecated: Some(Deprecated {
                                version: "7.0.0".to_string(),
                                description: "types are going away".to_string()
                            }),
                        },
                    ],
                },
                params: BTreeMap::new(),
                body: Some(Body {
                    description: Some("The search request".to_string()),
                    required: Some(false),
                    serialize: None
                }),
            },
        );

        let (enum_ty, enum_decl, enum_impl) = EnumBuilder::from(&endpoint).build();

        assert_eq!(ty_a("SearchUrlParts"), enum_ty);

        let expected_decl = quote!(
            #[derive(Debug, Clone, PartialEq)]
            #[doc = "Url parts for the Search API"]
            pub enum SearchUrlParts<'a> {
                None,
                Index(&'a [&'a str]),
                IndexType(&'a [&'a str], &'a [&'a str]),
            }
        );

        ast_eq(expected_decl, enum_decl);

        let expected_impl = quote!(
            impl<'a> SearchUrlParts<'a> {
                pub fn build(self) -> Cow<'static, str> {
                    match self {
                        SearchUrlParts::None => "/_search".into(),
                        SearchUrlParts::Index(ref index) => {
                            let index_str = index.join(",");
                            let mut p = String::with_capacity(9usize + index_str.len());
                            p.push_str("/");
                            p.push_str(index_str.as_ref());
                            p.push_str("/_search");
                            p.into()
                        }
                        SearchUrlParts::IndexType(ref index, ref ty) => {
                            let index_str = index.join(",");
                            let ty_str = ty.join(",");
                            let mut p = String::with_capacity(10usize + index_str.len() + ty_str.len());
                            p.push_str("/");
                            p.push_str(index_str.as_ref());
                            p.push_str("/");
                            p.push_str(ty_str.as_ref());
                            p.push_str("/_search");
                            p.into()
                        }
                    }
                }
            }
        );

        ast_eq(expected_impl, enum_impl);
    }
}
