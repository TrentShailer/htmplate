//! Derives for htmplate

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprLit, Fields, GenericParam, Generics, Lit, Type,
    TypeParamBound, parse_macro_input, parse_quote, spanned::Spanned,
};

/// Derive `HtmplateElement`
#[proc_macro_derive(HtmplateElement)]
pub fn derive_from_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Add required trait bounds depending on type.
    let generics = add_trait_bounds(input.generics, parse_quote!(FromStr));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(data_struct) = input.data else {
        panic!("HtmplateElement can only be derived on a struct")
    };

    let fields: Vec<syn::Field> = match data_struct.fields {
        Fields::Named(fields) => fields.named.into_iter().collect(),
        Fields::Unit => vec![],
        _ => {
            panic!(
                "HtmplateElement can only be derived on unit structs or a struct with named fields"
            )
        }
    };

    let attributes = fields.iter().map(|field| {
        let name_literal = &field
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_case(Case::Kebab);
        let description = get_doc(&field.attrs).expect("htmplate fields must have doc comments");
        let is_required = !is_option(&field.ty);

        quote_spanned! {field.span()=> htmplate::Attribute {
            name: #name_literal,
            description: #description,
            required: #is_required,
        }}
    });

    let get_fields = fields.iter().enumerate().map(|(index, field)| {
        let name = &field.ident;
        let name_literal = &field
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_case(Case::Kebab);

        let is_required = !is_option(&field.ty);

        let none_action = if is_required {
            quote! {
                error.missing_attributes.push(attributes[#index]);
            }
        } else {
            TokenStream::new()
        };

        quote_spanned! {field.span()=>
            let #name = match el.get_attribute(#name_literal) {
                Some(value) => match value.parse() {
                    Ok(value) => Some(value),
                    Err(_) => {
                        error.invalid_attributes.push(attributes[#index]);
                        None
                    }
                },
                None => {
                    #none_action
                    None
                }
            };
        }
    });

    let struct_fields = fields.iter().map(|field| {
        let name = &field.ident;

        let is_required = !is_option(&field.ty);

        let value = if is_required {
            quote! {#name.unwrap()}
        } else {
            quote! {#name}
        };

        quote_spanned! {field.span()=>
            #name: #value
        }
    });

    let tag = format!("htmplate\\:{}", name.to_string().to_case(Case::Kebab));

    let description = get_doc(&input.attrs).expect("An htmplate must have a doc comment");

    let implementation = quote! {
        impl #impl_generics htmplate::HtmplateElement for #name #ty_generics #where_clause {
            fn tag() -> &'static str {
                #tag
            }

            fn description() -> &'static str {
                #description
            }

            fn attributes() -> Vec<htmplate::Attribute> {
                vec![
                    #( #attributes ),*
                ]
            }

            fn from_element(el: &htmplate::lol_html::html_content::Element) -> Result<Self, htmplate::FromElementError> {
                let attributes = Self::attributes();

                let mut error = htmplate::FromElementError {
                    missing_attributes: vec![],
                    invalid_attributes: vec![],
                    element_tag: el.tag_name(),
                    element_location: htmplate::Location::Byte(el.source_location().bytes().start),
                };

                #( #get_fields )*

                if !error.missing_attributes.is_empty() || !error.invalid_attributes.is_empty() {
                    return Err(error);
                }

                Ok(Self {
                    #( #struct_fields ),*
                })
            }
        }
    };

    proc_macro::TokenStream::from(implementation)
}

// Add a bound to every type parameter T.
fn add_trait_bounds(mut generics: Generics, bounds: TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(bounds.clone());
        }
    }
    generics
}

fn get_doc(attrs: &[Attribute]) -> Option<Expr> {
    let mut macro_args: TokenStream = TokenStream::new();

    for (i, line) in attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .flat_map(|a| a.meta.require_name_value())
        .enumerate()
    {
        if i > 0 {
            macro_args.extend([quote!(, "\n",)]);
        }

        if let Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) = &line.value
        {
            if let Some(trimmed) = lit_str.value().strip_prefix(' ') {
                trimmed.to_tokens(&mut macro_args);
                continue;
            }
        }

        line.value.to_tokens(&mut macro_args);
    }

    if macro_args.is_empty() {
        None
    } else {
        Some(parse_quote!(::core::concat!(#macro_args)))
    }
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            let idents_of_path = typepath
                .path
                .segments
                .iter()
                .fold(String::new(), |mut acc, v| {
                    acc.push_str(&v.ident.to_string());
                    acc.push(':');
                    acc
                });
            vec!["Option:", "std:option:Option:", "core:option:Option:"]
                .into_iter()
                .find(|s| idents_of_path == *s)
                .and_then(|_| typepath.path.segments.last())
                .is_some()
        }
        _ => false,
    }
}

#[expect(unused)]
fn as_option(ty: &Type) -> Type {
    let ty = ty.clone();
    Type::Verbatim(quote! { ::core::option::Option<#ty> })
}
