extern crate proc_macro;

use darling::ast::Data;
use darling::FromDeriveInput;
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, DeriveInput, Ident, LitStr, Type};

#[derive(darling::FromDeriveInput)]
#[darling(attributes(from_values), supports(struct_named))]
struct FromValuesInput {
    ident: syn::Ident,
    generics: syn::Generics,
    output: Option<Type>,
    data: Data<(), FromValuesField>,
}

#[derive(darling::FromField)]
#[darling(forward_attrs(doc))]
struct FromValuesField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

#[proc_macro_derive(FromValues, attributes(from_values))]
pub fn derive_from_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input = FromValuesInput::from_derive_input(&input).unwrap();

    let t_value = quote! { ::nu_protocol::Value };
    let t_shell_error = quote! { ::nu_protocol::ShellError };
    let t_labelled_error = quote! { ::nu_protocol::LabeledError };
    let t_from_value = quote! { ::nu_plugin_helpers::FromValue };
    let t_from_values = quote! { ::nu_plugin_helpers::FromValues };
    let t_arg_signature = quote! { ::nu_plugin_helpers::ArgSignature };
    let t_result = quote! { ::std::result::Result };
    let t_vec = quote! { ::std::vec::Vec };

    // let fields = get_struct_fields(&input);
    let fields = input.data.take_struct().unwrap().fields;
    let from_values_fields = fields.iter().enumerate().map(|(arg_i, field)| {
        let ident = &field.ident;
        let ty = &field.ty;

        quote! {
            #ident: {
                let value = positional
                    .get(#arg_i)
                    .ok_or_else(||
                        #t_labelled_error::new(
                            format!(
                                "missing positional arg {} ({})",
                                #arg_i,
                                stringify!(#ident),
                            )
                        )
                    )?;

                <#ty as #t_from_value>::from_value(value)?
            },
        }
    });

    let arg_signatures = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        let description = field
            .attrs
            .iter()
            .map(|attr| {
                let doc_comment = &attr.meta.require_name_value().unwrap().value;
                let doc_comment = parse2::<LitStr>(doc_comment.into_token_stream()).unwrap();

                doc_comment.value().trim().to_string()
            })
            .join(" ");

        quote! {
            #t_arg_signature::new(
                stringify!(#ident),
                #description,
                <#ty as #t_from_value>::syntax_shape(),
            ),
        }
    });

    let ident = input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let output_type = match input.output {
        Some(output_type) => quote! { #output_type },
        None => {
            let mut generics = input.generics.clone();
            let value_lt_ident = Ident::new("value", Span::call_site());

            generics
                .lifetimes_mut()
                .for_each(|param| param.lifetime.ident = value_lt_ident.clone());

            let (_, type_generics, _) = generics.split_for_impl();

            quote! { #ident #type_generics }
        }
    };

    quote! {
        impl #impl_generics #t_from_values for #ident #type_generics #where_clause {
            type Output<'value> = #output_type;

            fn from_values(positional: &[#t_value]) -> #t_result<Self::Output<'_>, #t_shell_error> {
                #t_result::Ok(#ident {
                    #(#from_values_fields)*
                })
            }

            fn arg_signatures() -> #t_vec<#t_arg_signature> {
                ::std::vec![
                    #(#arg_signatures)*
                ]
            }
        }
    }
    .into()
}
