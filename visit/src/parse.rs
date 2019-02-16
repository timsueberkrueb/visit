use syn::visit::Visit;

pub fn get_visitor_trait_ident(file: &syn::File) -> proc_macro2::Ident {
    let mut visitor_trait_ident: Option<proc_macro2::Ident> = None;

    if file.attrs.len() > 1 {
        panic!("Multiple inner attributes or attributes other than `#[visitor_trait = \"VisitorTrait\"]` are not supported");
    }

    let attr = &file.attrs[0];

    if let Ok(meta) = attr.parse_meta() {
        if meta.name() == "visitor_trait" {
            if let syn::Meta::NameValue(meta_value) = meta {
                if meta_value.ident == "visitor_trait" {
                    if let syn::Lit::Str(lit_str) = meta_value.lit {
                        let ident =
                            syn::Ident::new(&lit_str.value(), proc_macro2::Span::call_site());
                        visitor_trait_ident = Some(ident);
                    }
                }
            }
        }
    }

    visitor_trait_ident.expect("Expected a `#[visitor_trait = \"VisitorTrait\"]` declaration.")
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
