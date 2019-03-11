use case::CaseExt;
use quote::quote;

use crate::parse::VisitorTraitConf;

pub fn generate_visitor_trait(
    visitor_trait_info: &VisitorTraitConf,
    structs: &[&syn::ItemStruct],
    enums: &[&syn::ItemEnum],
) -> proc_macro2::TokenStream {
    let visitor_trait_ident = &visitor_trait_info.ident;
    let visitor_trait_pub = if visitor_trait_info.public {
        quote! { pub }
    } else {
        quote! {}
    };

    let items: Vec<_> = structs
        .iter()
        .by_ref()
        .map(|s| GenericItem {
            ident: &s.ident,
            generics: &s.generics,
        })
        .chain(enums.iter().by_ref().map(|e| GenericItem {
            ident: &e.ident,
            generics: &e.generics,
        }))
        .collect();

    let mut idents = Vec::new();
    let mut visit_fn_idents = Vec::new();
    let mut param_idents = Vec::new();
    let mut param_generics = Vec::new();
    let mut param_where_clauses = Vec::new();

    for item in items {
        let visit_fn_ident = visitor_item_fn_ident(&item.ident);
        let param_string = format!("_{}", item.ident.to_string().to_snake());
        let param_ident = syn::Ident::new(&param_string, proc_macro2::Span::call_site());
        let param_generics_stream = if item.generics.params.is_empty() {
            quote! {}
        } else {
            let generic_params = &item.generics.params;
            quote! { <#generic_params> }
        };
        let item_ident = &item.ident;
        idents.push(quote! { #item_ident });
        visit_fn_idents.push(visit_fn_ident);
        param_idents.push(param_ident);
        param_generics.push(param_generics_stream);
        param_where_clauses.push(&item.generics.where_clause);
    }

    let param_generics_2 = param_generics.clone();

    quote! {
        #visitor_trait_pub trait #visitor_trait_ident {
            #(
                fn #visit_fn_idents #param_generics (&mut self, #param_idents: &#idents #param_generics_2)
                #param_where_clauses
                {}
            )*
        }
    }
}

/// Helper struct to represent either a struct or an enum item
struct GenericItem<'a> {
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
}

pub fn generate_accept_visitor_trait(
    visitor_trait_info: &VisitorTraitConf,
    accept_trait_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    let visitor_trait_ident = &visitor_trait_info.ident;
    let visitor_trait_pub = if visitor_trait_info.public {
        quote! { pub }
    } else {
        quote! {}
    };

    quote! {
        #visitor_trait_pub trait #accept_trait_ident {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V);
        }
    }
}

pub fn generate_accept_visitor_impls(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
    macro_rules! impl_empty_accept {
        ($t:ty) => {
            quote! {
                impl #accept_trait_ident for $t {
                    fn accept<V: #visitor_trait_ident>(&self, _visitor: &mut V) {}
                }
            }
        };
    }

    let mut stream = quote! {
        impl<TItem> #accept_trait_ident for [TItem]
        where
            TItem: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                for item in self.iter().by_ref() {
                    item.accept(visitor);
                }
            }
        }

        impl<TItem> #accept_trait_ident for Vec<TItem>
        where
            TItem: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                for item in self.iter().by_ref() {
                    item.accept(visitor);
                }
            }
        }

        impl<TItem> #accept_trait_ident for std::collections::HashSet<TItem>
        where
            TItem: #accept_trait_ident + Eq + std::hash::Hash,
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                for item in self.iter().by_ref() {
                    item.accept(visitor);
                }
            }
        }

        impl<T> #accept_trait_ident for Option<T>
        where
            T: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                if let Some(inner) = self {
                    inner.accept(visitor);
                }
            }
        }

        impl<T> #accept_trait_ident for Box<T>
        where
            T: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                <Self as std::ops::Deref>::deref(self).accept(visitor);
            }
        }

        impl<T> #accept_trait_ident for std::rc::Rc<T>
        where
            T: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                <Self as std::ops::Deref>::deref(self).accept(visitor);
            }
        }

        impl<T> #accept_trait_ident for std::sync::Arc<T>
        where
            T: #accept_trait_ident
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                <Self as std::ops::Deref>::deref(self).accept(visitor);
            }
        }
    };

    // Ignore primitive datatypes by providing empty AcceptVisitor implementations

    stream.extend(impl_empty_accept!(u8));
    stream.extend(impl_empty_accept!(u16));
    stream.extend(impl_empty_accept!(u32));
    stream.extend(impl_empty_accept!(u64));
    stream.extend(impl_empty_accept!(u128));

    stream.extend(impl_empty_accept!(i8));
    stream.extend(impl_empty_accept!(i16));
    stream.extend(impl_empty_accept!(i32));
    stream.extend(impl_empty_accept!(i64));
    stream.extend(impl_empty_accept!(i128));

    stream.extend(impl_empty_accept!(usize));
    stream.extend(impl_empty_accept!(isize));

    stream.extend(impl_empty_accept!(f32));
    stream.extend(impl_empty_accept!(f64));

    stream.extend(impl_empty_accept!(bool));

    stream.extend(impl_empty_accept!(String));
    stream.extend(impl_empty_accept!(&str));

    stream
}

pub fn generate_accept_impl_for_struct(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
    item_struct: &syn::ItemStruct,
) -> proc_macro2::TokenStream {
    let generics_params = &item_struct.generics.params;
    let generics_params = if generics_params.is_empty() {
        quote! {}
    } else {
        quote! { <#generics_params> }
    };
    let generics_where_clause = &item_struct.generics.where_clause;

    let struct_ident = &item_struct.ident;
    let fn_ident = visitor_item_fn_ident(struct_ident);

    let field_idents: Vec<_> = match &item_struct.fields {
        syn::Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident}
            })
            .collect(),
        syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| quote! { #i })
            .collect(),
        syn::Fields::Unit => Vec::new(),
    };

    quote! {
        impl #generics_params #accept_trait_ident for #struct_ident #generics_params
        #generics_where_clause
        {
            fn accept<V: #visitor_trait_ident>(&self, visitor: &mut V) {
                #(
                    self.#field_idents.accept(visitor);
                )*
                visitor.#fn_ident(self);
            }
        }
    }
}

pub fn generate_accept_impl_for_enum(
    visitor_trait_ident: &syn::Ident,
    accept_trait_ident: &syn::Ident,
    item_enum: &syn::ItemEnum,
) -> proc_macro2::TokenStream {
    let enum_ident = &item_enum.ident;
    let fn_ident = visitor_item_fn_ident(enum_ident);
    let generics_params = &item_enum.generics.params;
    let generics_params = if generics_params.is_empty() {
        quote! {}
    } else {
        quote! { <#generics_params> }
    };
    let generics_where_clause = &item_enum.generics.where_clause;

    let mut match_body = proc_macro2::TokenStream::new();

    for variant in item_enum.variants.iter().by_ref() {
        let variant_ident = &variant.ident;
        let match_arm = match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let field_idents: Vec<_> = fields_named
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect();
                let field_idents_inner = field_idents.clone();
                quote! {
                    #enum_ident::#variant_ident { #(#field_idents),* } => {
                        #(
                            #field_idents_inner.accept(visitor);
                        )*
                    },
                }
            }
            syn::Fields::Unnamed(fields_unamed) => {
                let field_idents: Vec<_> = fields_unamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        syn::Ident::new(&format!("x{}", i), proc_macro2::Span::call_site())
                    })
                    .collect();
                let field_idents_inner = field_idents.clone();
                quote! {
                    #enum_ident::#variant_ident ( #(#field_idents),* ) => {
                        #(
                            #field_idents_inner.accept(visitor);
                        )*
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #enum_ident::#variant_ident => {},
                }
            }
        };
        match_body.extend(match_arm);
    }

    quote! {
        impl #generics_params #accept_trait_ident for #enum_ident #generics_params
        #generics_where_clause
        {
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
