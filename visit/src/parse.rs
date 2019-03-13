use std::collections::HashSet;

use darling::FromMeta;
use syn::visit::Visit;

pub fn get_visitor_trait_configs(file: &syn::File) -> Vec<VisitorTraitConf> {
    let mut names = HashSet::with_capacity(file.attrs.len());

    file.attrs
        .iter()
        .by_ref()
        .map(|attr| attr.parse_meta().expect("Failed to parse inner attribute"))
        .filter(|meta| meta.name() == "visitor" || meta.name() == "hierarchical_visitor")
        .map(|meta| {
            if meta.name() == "visitor" || meta.name() == "hierarchical_visitor" {
                let mut conf = VisitorTraitConf::from_meta(&meta)
                    .unwrap_or_else(|_| panic!("Invalid synatax in `{}` attribute", meta.name()));
                let name_string = conf.ident.to_string();
                if names.contains(&name_string) {
                    panic!("Visitor `{}` defined more than once", name_string);
                }
                names.insert(name_string);
                conf.hierarchical = meta.name() == "hierarchical_visitor";
                conf
            } else {
                panic!("Unexpect inner attribute `{}`", meta.name());
            }
        })
        .collect()
}

#[derive(Debug, FromMeta)]
pub struct VisitorTraitConf {
    #[darling(skip)]
    pub hierarchical: bool,
    #[darling(default)]
    pub public: bool,
    #[darling(rename = "name")]
    pub ident: proc_macro2::Ident,
}

impl VisitorTraitConf {
    pub fn accept_trait_ident(&self) -> syn::Ident {
        let visitor_trait_string = self.ident.to_string();
        let accept_trait_string = format!("Accept{}", visitor_trait_string);
        syn::Ident::new(&accept_trait_string, proc_macro2::Span::call_site())
    }
}

pub struct ASTVisitor<'ast> {
    pub structs: Vec<&'ast syn::ItemStruct>,
    pub enums: Vec<&'ast syn::ItemEnum>,
}

impl<'ast> ASTVisitor<'ast> {
    pub fn new() -> Self {
        Self {
            structs: Vec::new(),
            enums: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ASTVisitor<'ast> {
    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        self.structs.push(s);
    }

    fn visit_item_enum(&mut self, e: &'ast syn::ItemEnum) {
        self.enums.push(e);
    }
}
