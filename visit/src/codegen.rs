use case::CaseExt;
use quote::quote;

use crate::iter_utils::Unzip3;

pub fn generate_visitor_trait(
    visitor_trait_ident: &syn::Ident,
    structs: &[&syn::ItemStruct],
    enums: &[&syn::ItemEnum],
) -> proc_macro2::TokenStream {
    let struct_idents: Vec<_> = structs.iter().by_ref().map(|s| &s.ident).collect();

    let enum_idents: Vec<_> = enums.iter().by_ref().map(|e| &e.ident).collect();

    let (idents, visit_fn_idents, param_idents): (
        Vec<&proc_macro2::Ident>,
        Vec<proc_macro2::Ident>,
        Vec<proc_macro2::Ident>,
    ) = struct_idents
        .iter()
        .chain(enum_idents.iter())
        .by_ref()
        .map(|ident| {
            let visit_fn_ident = visitor_item_fn_ident(&ident);
            let param_string = format!("_{}", ident.to_string().to_snake());
            let param_ident = syn::Ident::new(&param_string, proc_macro2::Span::call_site());
            (ident, visit_fn_ident, param_ident)
        })
        .unzip_3();

    quote! {
        trait #visitor_trait_ident {
            #(
                fn #visit_fn_idents(&mut self, #param_idents: &#idents) {}
            )*
        }
    }
}

pub fn generate_accept_visitor_trait(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        trait #accept_trait_ident {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V);
        }
    }
}

pub fn generate_accept_impl_for_struct(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
    item_struct: &syn::ItemStruct,
) -> proc_macro2::TokenStream {
    let struct_ident = &item_struct.ident;
    let fn_ident = visitor_item_fn_ident(struct_ident);

    if let syn::Fields::Named(fields_named) = &item_struct.fields {
        let field_idents = fields_named.named.iter().map(|f| f.ident.clone().unwrap());
        quote! {
            impl #accept_trait_ident for #struct_ident {
                fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                    #(
                        self.#field_idents.accept(visitor);
                    )*
                    visitor.#fn_ident(self);
                }
            }
        }
    } else {
        panic!("Only structs with named fields are supported, currently.");
    }
}

pub fn generate_accept_impl_for_enum(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
    item_enum: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_ident = &item_enum.ident;
    let fn_ident = visitor_item_fn_ident(enum_ident);

    let mut match_body = proc_macro2::TokenStream::new();

    for variant in item_enum.variants.iter().by_ref() {
        let variant_ident = &variant.ident;
        if let syn::Fields::Named(fields_named) = &variant.fields {
            let field_idents: Vec<_> = fields_named
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect();
            let field_idents_inner = field_idents.clone();
            let match_arm = quote! {
                #enum_ident::#variant_ident { #(#field_idents),* } => {
                    #(
                        #field_idents_inner.accept(visitor);
                    )*
                },
            };

            match_body.extend(match_arm);
        } else {
            panic!("Only enums with named fields are supported, currently.");
        }
    }

    quote! {
        impl #accept_trait_ident for #enum_ident {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                match self {
                    #match_body
                }
                visitor.#fn_ident(self);
            }
        }
    }
}

pub fn accept_visitor_trait_ident(visitor_trait_ident: &syn::Ident) -> syn::Ident {
    let visitor_trait_string = visitor_trait_ident.to_string();
    let accept_trait_string = format!("Accept{}", visitor_trait_string);
    syn::Ident::new(&accept_trait_string, proc_macro2::Span::call_site())
}

fn visitor_item_fn_ident(item_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    const PREFIX: &str = "visit";

    let ident_string = item_ident.to_string();
    let ident_snake = ident_string.to_snake();
    let prefixed_string = format!("{}_{}", PREFIX, ident_snake);

    syn::Ident::new(&prefixed_string, proc_macro2::Span::call_site())
}
