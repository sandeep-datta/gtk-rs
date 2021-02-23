// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{ItemFn, ReturnType, Type, TypePath, TypeReference, TypeSlice};

pub fn impl_lazy_static(input: ItemFn) -> TokenStream {
    let vis = &input.vis;
    let block = &input.block;
    let name = &input.sig.ident;
    let output = &input.sig.output;

    // Extract the function return_type
    let return_type = match output {
        ReturnType::Type(_, ret_type) => ret_type,
        syn::ReturnType::Default => {
            abort_call_site!("#[lazy_static] expects a return type")
        }
    };
    let output_type = match &**return_type {
        Type::Reference(TypeReference { elem, .. }) => match &**elem {
            Type::Slice(TypeSlice { elem, .. }) => {
                if let Type::Path(TypePath { path, .. }) = &**elem {
                    quote! { #path }
                } else {
                    quote! {}
                }
            }
            _ => abort_call_site!("#[lazy_static] expects a [T] return type"),
        },
        _ => abort_call_site!("#[lazy_static] expects a &'static lifetime"),
    };

    (quote! {
        #vis fn #name() #output {
            static STATIC: once_cell::sync::Lazy<Vec<#output_type>> = once_cell::sync::Lazy::new(|| {
                #block
            });
            STATIC.as_ref()
        }
    }).into()
}
