use crate::api_generator::{
    code_gen, code_gen::url::enum_builder::EnumBuilder, code_gen::*, ApiEndpoint, HttpMethod, Type,
    TypeKind,
};
use inflector::Inflector;
use quote::{ToTokens, Tokens};
use std::{collections::BTreeMap, str};
use syn::{Field, FieldValue, ImplItem};

/// Builder that generates the AST for a request builder struct
pub struct RequestBuilder<'a> {
    /// The name of the API to which the generated struct relates
    name: &'a str,
    /// The name of the generated struct
    builder_name: &'a str,
    /// Parameters that are common to all APIs
    common_params: &'a BTreeMap<String, Type>,
    /// The endpoint to which the API relates
    endpoint: &'a ApiEndpoint,
    /// The builder for the Url parts enum related to the API
    enum_builder: EnumBuilder<'a>,
    /// Whether the API exists on the root client or on a namespace client
    is_root_method: bool,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(
        name: &'a str,
        builder_name: &'a str,
        common_params: &'a BTreeMap<String, Type>,
        endpoint: &'a ApiEndpoint,
        is_root_method: bool,
    ) -> Self {
        let mut enum_builder = EnumBuilder::new(builder_name.to_pascal_case().as_ref());
        for path in &endpoint.url.paths {
            enum_builder = enum_builder.with_path(path);
        }

        RequestBuilder {
            name,
            builder_name,
            common_params,
            endpoint,
            enum_builder,
            is_root_method,
        }
    }

    /// Create the AST for an expression that assigns a HttpMethod value
    fn create_method_expression(builder_name: &str, endpoint: &ApiEndpoint) -> syn::Expr {
        let methods = {
            let mut m = Vec::new();
            for path in &endpoint.url.paths {
                for method in &path.methods {
                    if !m.contains(&method) {
                        m.push(method);
                    }
                }
            }
            m
        };

        match methods.len() {
            1 => {
                let method = *methods.first().unwrap();
                let mut tokens = Tokens::new();
                method.to_tokens(&mut tokens);
                parse_expr(tokens)
            }
            _ => match methods.as_slice() {
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
                let field = Self::create_struct_field((param_name, param_type));
                let field_rename = lit(param_name);
                let skip_serializing_if = lit("Option::is_none");
                if param_type.ty == TypeKind::List {
                    let serialize_with = lit("crate::client::serialize_coll_qs");
                    quote! {
                        #[serde(rename = #field_rename, serialize_with = #serialize_with, skip_serializing_if = #skip_serializing_if)]
                        #field
                    }
                }
                else {
                    quote! {
                        #[serde(rename = #field_rename, skip_serializing_if = #skip_serializing_if)]
                        #field
                    }
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
                    struct #query_struct_typ<'a> {
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

    /// Creates the AST for a ctor new fn for a builder struct
    fn create_new_fn(
        builder_ident: &syn::Ident,
        enum_builder: &EnumBuilder,
        default_fields: &[&syn::Ident],
    ) -> Tokens {
        let (enum_ty, _, _) = enum_builder.clone().build();
        let default_fields = Self::create_default_fields(default_fields);
        if enum_builder.contains_single_parameterless_part() {
            quote!(
                 pub fn new(client: Elasticsearch) -> Self {
                    #builder_ident {
                        client,
                        parts: #enum_ty::None,
                        #(#default_fields),*,
                    }
                }
            )
        } else {
            quote!(
                 pub fn new(client: Elasticsearch, parts: #enum_ty) -> Self {
                    #builder_ident {
                        client,
                        parts,
                        #(#default_fields),*,
                    }
                }
            )
        }
    }

    /// Creates the AST for the body fn for a builder struct that supports sending a body
    fn create_body_fn(
        builder_name: &str,
        builder_ident: &syn::Ident,
        default_fields: &[&syn::Ident],
    ) -> syn::ImplItem {
        let fields: Vec<FieldValue> = default_fields
            .iter()
            .filter(|&&part| part != &ident("body"))
            .map(|&part| syn::FieldValue {
                attrs: vec![],
                ident: ident(part),
                expr: syn::ExprKind::Path(
                    None,
                    path_none(ident(format!("self.{}", part.as_ref())).as_ref()),
                )
                .into(),
                is_shorthand: false,
            })
            .collect();

        syn::ImplItem {
            ident: ident("body<T>"),
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
                            syn::FnArg::SelfValue(syn::Mutability::Immutable),
                            syn::FnArg::Captured(
                                syn::Pat::Path(None, path_none("body")),
                                syn::parse_type("T").unwrap(),
                            ),
                        ],
                        output: syn::FunctionRetTy::Ty(code_gen::ty(
                            format!("{}<'a, T> where T: Serialize", &builder_name).as_ref(),
                        )),
                        variadic: false,
                    },
                    generics: generics_none(),
                },
                // generates a fn body of the form
                // --------
                // <builder_name> {
                //     body: body,
                //     ... assign rest of fields
                // }
                // ---------
                syn::Block {
                    stmts: vec![syn::Stmt::Expr(Box::new(parse_expr(quote!(
                            #builder_ident {
                                client: self.client,
                                parts: self.parts,
                                body: Some(body),
                                #(#fields),*,
                            }
                    ))))],
                },
            ),
        }
    }

    /// Creates the AST for a builder fn for a builder impl
    fn create_impl_fn(f: (&String, &Type)) -> syn::ImplItem {
        let name = valid_name(&f.0).to_lowercase();
        let impl_ident = ident(&name);
        let field_ident = ident(&name);
        let value_ident = ident(&name);
        let ty = typekind_to_ty(&f.0, f.1.ty, true);
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
                            syn::FnArg::Captured(
                                syn::Pat::Path(None, path_none(name.as_str())),
                                ty,
                            ),
                        ],
                        output: syn::FunctionRetTy::Ty(code_gen::ty("Self")),
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
                            quote!(self.#field_ident = Some(#value_ident)),
                        ))),
                        syn::Stmt::Expr(Box::new(parse_expr(quote!(self)))),
                    ],
                },
            ),
        }
    }

    /// creates the AST for a builder struct
    fn create_builder_struct(
        builder_name: &str,
        endpoint: &ApiEndpoint,
        common_params: &BTreeMap<String, Type>,
        enum_builder: &EnumBuilder,
    ) -> Tokens {
        // TODO: lazy_static! for this?
        let common_fields: Vec<Field> = common_params
            .iter()
            .map(Self::create_struct_field)
            .collect();

        // TODO: lazy_static! for this?
        let common_builder_fns: Vec<ImplItem> =
            common_params.iter().map(Self::create_impl_fn).collect();

        let supports_body = endpoint.supports_body();
        let builder_ident = ident(builder_name);
        let (enum_ty, enum_struct, enum_impl) = enum_builder.clone().build();

        // collect all the fields for the builder struct. Start with url parameters
        let mut fields: Vec<Field> = endpoint
            .params
            .iter()
            .map(Self::create_struct_field)
            .collect();

        if supports_body {
            fields.push(syn::Field {
                ident: Some(ident("body")),
                vis: syn::Visibility::Inherited,
                attrs: vec![],
                ty: syn::parse_type("Option<B>").unwrap(),
            })
        }

        // Combine common fields with struct fields, sort and deduplicate
        // clone common_fields, since quote!() consumes the Vec<Field>
        fields.append(&mut common_fields.to_vec().clone());
        fields.sort_by(|a, b| a.ident.cmp(&b.ident));
        fields.dedup_by(|a, b| a.ident.eq(&b.ident));

        let default_fields = {
            fields
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>()
        };

        // collect all the functions for the builder struct
        let mut builder_fns: Vec<ImplItem> =
            endpoint.params.iter().map(Self::create_impl_fn).collect();

        // add a body impl if supported
        if supports_body {
            let body_fn = Self::create_body_fn(&builder_name, &builder_ident, &default_fields);
            builder_fns.push(body_fn);
        }

        // Combine common fns with builder fns, sort and deduplicate.
        // clone is required, since quote!() consumes the Vec<Item>
        builder_fns.append(&mut common_builder_fns.to_vec().clone());
        builder_fns.sort_by(|a, b| a.ident.cmp(&b.ident));
        builder_fns.dedup_by(|a, b| a.ident.eq(&b.ident));

        let new_fn = Self::create_new_fn(&builder_ident, enum_builder, &default_fields);

        let method_expr = Self::create_method_expression(&builder_name, &endpoint);

        let query_string_params = {
            let mut p = endpoint.params.clone();
            p.append(&mut common_params.clone());
            p
        };

        let query_string_expr = Self::create_query_string_expression(&query_string_params);

        let body_expr = {
            if supports_body {
                quote!(self.body)
            } else {
                quote!(Option::<()>::None)
            }
        };

        let (builder_expr, builder_impl) = {
            if supports_body {
                (
                    quote!(#builder_ident<'a, B>),
                    quote!(impl<'a, B> #builder_ident<'a, B> where B: Serialize),
                )
            } else {
                (
                    quote!(#builder_ident<'a>),
                    quote!(impl<'a> #builder_ident<'a>),
                )
            }
        };

        let api_name_for_docs = split_on_pascal_case(builder_name);
        let builder_doc = lit(format!("Request builder for the {} API", api_name_for_docs));
        let send_doc = lit(format!(
            "Creates an asynchronous request to the {} API that can be awaited",
            api_name_for_docs
        ));

        quote! {
            #enum_struct

            #enum_impl

            #[derive(Clone, Debug)]
            #[doc = #builder_doc]
            pub struct #builder_expr {
                client: Elasticsearch,
                parts: #enum_ty,
                #(#fields),*,
            }

            #builder_impl {
                #new_fn
                #(#builder_fns)*

                #[doc = #send_doc]
                pub async fn send(self) -> Result<ElasticsearchResponse, ElasticsearchError> {
                      let path = self.parts.build();
                      let method = #method_expr;
                      let query_string = #query_string_expr;
                      let body = #body_expr;
                      let response = self.client.send(method, &path, query_string.as_ref(), body).await?;
                      Ok(response)
                }
            }
        }
    }

    /// Creates the AST for a fn that returns a new instance of a builder struct
    /// from the root or namespace client
    fn create_builder_struct_ctor_fns(
        name: &str,
        builder_name: &str,
        endpoint: &ApiEndpoint,
        is_root_method: bool,
        enum_builder: &EnumBuilder,
    ) -> Tokens {
        let builder_ident = ident(builder_name);

        let (fn_name, builder_ident_ret) = {
            let i = ident(name);
            let b = builder_ident.clone();
            if endpoint.supports_body() {
                (quote!(#i<'a>), quote!(#b<'a, ()>))
            } else {
                (quote!(#i<'a>), quote!(#b<'a>))
            }
        };

        let method_doc = match &endpoint.documentation.description {
            Some(description) => Some(doc(description.into())),
            _ => None,
        };

        let clone_expr = {
            if is_root_method {
                quote!(self.clone())
            } else {
                quote!(self.client.clone())
            }
        };

        if enum_builder.contains_single_parameterless_part() {
            quote!(
                #method_doc
                pub fn #fn_name(&self) -> #builder_ident_ret {
                    #builder_ident::new(#clone_expr)
                }
            )
        } else {
            let (enum_ty, _, _) = enum_builder.clone().build();
            quote!(
                #method_doc
                pub fn #fn_name(&self, parts: #enum_ty) -> #builder_ident_ret {
                    #builder_ident::new(#clone_expr, parts)
                }
            )
        }
    }

    /// Creates the AST for a field for a struct
    fn create_struct_field(f: (&String, &Type)) -> syn::Field {
        syn::Field {
            ident: Some(ident(valid_name(&f.0).to_lowercase())),
            vis: syn::Visibility::Inherited,
            attrs: vec![],
            ty: typekind_to_ty(&f.0, f.1.ty, false),
        }
    }

    /// Creates the AST for field values initialized with a default value.
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

    /// builds the AST that represent the builder structs
    /// and the ctor function for the builder struct on the namespace client
    pub fn build(self) -> (Tokens, Tokens) {
        let builder_struct = Self::create_builder_struct(
            self.builder_name,
            self.endpoint,
            self.common_params,
            &self.enum_builder,
        );

        let ctor_fn = Self::create_builder_struct_ctor_fns(
            self.name,
            self.builder_name,
            self.endpoint,
            self.is_root_method,
            &self.enum_builder,
        );

        (builder_struct, ctor_fn)
    }
}
