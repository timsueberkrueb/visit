use case::CaseExt;
use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::VisitorTraitConf;

pub struct Generator<'ast> {
    structs: Vec<&'ast syn::ItemStruct>,
    enums: Vec<&'ast syn::ItemEnum>,
}

impl<'ast> Generator<'ast> {
    pub fn new(structs: Vec<&'ast syn::ItemStruct>, enums: Vec<&'ast syn::ItemEnum>) -> Self {
        Self { structs, enums }
    }

    pub fn generate(&self, conf: &VisitorTraitConf) -> TokenStream {
        let visitor_trait_gen = self.generate_visitor_trait(&conf);
        let accept_trait_gen = generate_accept_visitor_trait(&conf);
        let accept_trait_impls = generate_accept_visitor_impls(conf);

        let mut accept_impls = TokenStream::new();
        for item_struct in self.structs.iter().by_ref() {
            let stream = generate_accept_impl_for_struct(conf, &item_struct);
            accept_impls.extend(stream);
        }
        for item_enum in self.enums.iter().by_ref() {
            let stream = generate_accept_impl_for_enum(conf, &item_enum);
            accept_impls.extend(stream);
        }
        quote! {
            #visitor_trait_gen
            #accept_trait_gen
            #accept_trait_impls
            #accept_impls
        }
    }

    fn generate_visitor_trait(&self, conf: &VisitorTraitConf) -> TokenStream {
        let visitor_trait_ident = &conf.ident;
        let visitor_trait_pub = if conf.public {
            quote! { pub }
        } else {
            quote! {}
        };

        let items = generalize_items(&self.structs, &self.enums);
        let function_defs = generate_functions_defs_for(items, |ident| visit_fn_ident(ident));

        quote! {
            #visitor_trait_pub trait #visitor_trait_ident {
                #function_defs
            }
        }
    }
}

fn generalize_items<'a>(
    structs: &[&'a syn::ItemStruct],
    enums: &[&'a syn::ItemEnum],
) -> Vec<GenericItem<'a>> {
    structs
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
        .collect()
}

fn generate_functions_defs_for<F>(items: Vec<GenericItem>, map_name: F) -> TokenStream
where
    F: Fn(&proc_macro2::Ident) -> proc_macro2::Ident,
{
    let mut idents = Vec::new();
    let mut visit_fn_idents = Vec::new();
    let mut param_idents = Vec::new();
    let mut param_generics = Vec::new();
    let mut param_where_clauses = Vec::new();

    for item in items {
        let visit_fn_ident = map_name(&item.ident);
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
        #(
            fn #visit_fn_idents #param_generics (&mut self, #param_idents: &#idents #param_generics_2)
            #param_where_clauses
            {}
        )*
    }
}

/// Helper struct to represent either a struct or an enum item
struct GenericItem<'a> {
    ident: &'a syn::Ident,
    generics: &'a syn::Generics,
}

fn generate_accept_visitor_trait(conf: &VisitorTraitConf) -> TokenStream {
    let visitor_trait_ident = &conf.ident;
    let accept_trait_ident = &conf.accept_trait_ident();
    let visitor_trait_pub = if conf.public {
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

fn generate_accept_impl_for_struct(
    conf: &VisitorTraitConf,
    item_struct: &syn::ItemStruct,
) -> TokenStream {
    let visitor_trait_ident = &conf.ident;
    let accept_trait_ident = conf.accept_trait_ident();

    let generics_params = &item_struct.generics.params;
    let generics_params = if generics_params.is_empty() {
        quote! {}
    } else {
        quote! { <#generics_params> }
    };
    let generics_where_clause = &item_struct.generics.where_clause;

    let struct_ident = &item_struct.ident;
    let fn_ident = visit_fn_ident(struct_ident);

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

fn generate_accept_impl_for_enum(
    conf: &VisitorTraitConf,
    item_enum: &syn::ItemEnum,
) -> TokenStream {
    let visitor_trait_ident = &conf.ident;
    let accept_trait_ident = conf.accept_trait_ident();
    let enum_ident = &item_enum.ident;
    let fn_ident = visit_fn_ident(enum_ident);
    let generics_params = &item_enum.generics.params;
    let generics_params = if generics_params.is_empty() {
        quote! {}
    } else {
        quote! { <#generics_params> }
    };
    let generics_where_clause = &item_enum.generics.where_clause;

    let mut match_body = TokenStream::new();

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

fn generate_accept_visitor_impls(conf: &VisitorTraitConf) -> TokenStream {
    let visitor_trait_ident = &conf.ident;
    let accept_trait_ident = conf.accept_trait_ident();

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

fn visit_fn_ident(item_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    prefixed_fn_ident("visit", item_ident)
}

fn enter_fn_ident(item_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    prefixed_fn_ident("enter", item_ident)
}

fn leave_fn_ident(item_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    prefixed_fn_ident("leave", item_ident)
}

fn prefixed_fn_ident(prefix: &'static str, item_ident: &proc_macro2::Ident) -> proc_macro2::Ident {
    let ident_string = item_ident.to_string();
    let ident_snake = ident_string.to_snake();
    let prefixed_string = format!("{}_{}", prefix, ident_snake);

    syn::Ident::new(&prefixed_string, proc_macro2::Span::call_site())
}
