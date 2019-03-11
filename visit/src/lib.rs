#![recursion_limit = "256"]

extern crate proc_macro;

use quote::quote;

mod codegen;
mod parse;

use syn::visit::Visit;

/// Procedural macro to automatically generate code for the
/// [Visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern)
///
/// # Example
///
/// ```
/// use visit::visit;
///
/// visit! {
///     #![visitor(name = "Visitor")]
///
///     struct Bar {
///         a: Child,
///         b: Child,
///     }
///
///     struct Child {}
/// }
///
/// struct MyVisitor;
///
/// impl Visitor for MyVisitor {
///     fn visit_child(&mut self, child: &Child) {
///         // Do something cool
///     }
/// }
///
/// # fn main() {}
/// ```
///
/// Use the `accept` method on the data structure you want to visit:
///
/// ```
/// # use simple::*;
/// #
/// let mut visitor = MyVisitor {};
/// let root = Bar { a: Child {}, b: Child {} };
/// root.accept(&mut visitor);
/// ```
///
/// # Attributes
///
/// ```ignore
/// #![visitor(name = "Visitor", public = true)]
/// ```
///
/// Generate a visitor trait with a given name. If `public` is true, the generated trait will be `pub`. `public` is
/// `false` by default.
///
/// # Details
///
/// visit automatically generates visitor traits as declared by the `#![visitor]` attributes:
///
/// ```
/// # use simple::{Bar, Child};
/// #
/// trait Visitor {
///     fn visit_bar(&mut self, bar: &Bar) {}
///     fn visit_child(&mut self, child: &Child) {}
///     // ...
/// }
/// ```
///
/// This trait specifies `visit` methods for all supported items (structs and enums) contained inside the `visit!` macro
/// block.
/// It also provides empty default implementations for all methods so you only need to implement the `visit_*` methods
/// that are relevant to your current use case.
///
/// visit also generates an accept visitor trait. It is named `AcceptVisitor` where `Visitor` will be replaced by the
/// name specified using the respective `#![visitor]` attribute.
///
/// ```
/// # use simple::Visitor;
/// #
/// trait AcceptVisitor {
///     fn accept<V: Visitor>(&self, visitor: &mut V);
/// }
/// ```
///
/// This trait gets automatically implemented for all items contained inside the `visit!` macro block. For example, a
/// trait implementation generated for `Bar` could look like this:
///
/// ```ignore
/// impl AcceptVisitor for Bar {
///     fn accept<V: Visitor>(&self, visitor: &mut V) {
///         self.a.accept(visitor);
///         self.b.accept(visitor);
///         visitor.visit_bar(self);
///     }
/// }
/// ```
///
/// visit also generates some default implementations for common collections and `Option<T>`. Primitive types are
/// ignored (visit generates an empty accept trait implementation for them).
///
#[proc_macro]
pub fn visit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut file: syn::File = syn::parse2(input.into()).unwrap();
    let visitor_configs = parse::get_visitor_trait_configs(&file);
    // Inner attributes are not stable yet, therefore we have to cut them out
    file.attrs = Vec::new();

    let mut visitor = parse::ASTVisitor::new();
    visitor.visit_file(&file);

    let mut result = proc_macro2::TokenStream::new();

    for conf in visitor_configs {
        let token_stream = generate_code(&visitor, &conf);
        result.extend(token_stream);
    }

    let result = quote! {
        #file
        #result
    };

    result.into()
}

fn generate_code(
    visitor: &parse::ASTVisitor,
    conf: &parse::VisitorTraitConf,
) -> proc_macro2::TokenStream {
    let visitor_trait_gen =
        codegen::generate_visitor_trait(&conf, &visitor.structs, &visitor.enums);
    let accept_trait_ident = codegen::accept_visitor_trait_ident(&conf.ident);
    let accept_trait_gen = codegen::generate_accept_visitor_trait(&conf, &accept_trait_ident);
    let accept_trait_impls =
        codegen::generate_accept_visitor_impls(&conf.ident, &accept_trait_ident);

    let mut accept_impls = proc_macro2::TokenStream::new();
    for item_struct in visitor.structs.iter().by_ref() {
        let stream = codegen::generate_accept_impl_for_struct(
            &conf.ident,
            &accept_trait_ident,
            &item_struct,
        );
        accept_impls.extend(stream);
    }
    for item_enum in visitor.enums.iter().by_ref() {
        let stream =
            codegen::generate_accept_impl_for_enum(&conf.ident, &accept_trait_ident, &item_enum);
        accept_impls.extend(stream);
    }
    quote! {
        #visitor_trait_gen
        #accept_trait_gen
        #accept_trait_impls
        #accept_impls
    }
}
