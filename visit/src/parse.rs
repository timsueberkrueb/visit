use std::collections::HashSet;

use darling::FromMeta;
use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::diag::*;

pub fn get_visitor_trait_configs(file: &syn::File) -> Result<Vec<VisitorTraitConf>, Diagnostic> {
    let mut parser = FileParser {
        names: HashSet::with_capacity(file.attrs.len()),
    };

    parser.parse_file(file)
}

struct FileParser {
    names: HashSet<String>,
}

impl FileParser {
    fn parse_file(&mut self, file: &syn::File) -> Result<Vec<VisitorTraitConf>, Diagnostic> {
        let mut confs = Vec::with_capacity(file.attrs.len());
        for attr in file.attrs.iter().by_ref() {
            let conf = self.parse_attribute(attr)?;
            confs.push(conf);
        }
        Ok(confs)
    }

    fn parse_attribute(&mut self, attr: &syn::Attribute) -> Result<VisitorTraitConf, Diagnostic> {
        let meta = attr
            .parse_meta()
            .map_err(|_| attr.span().error("Failed to parse inner attribute"))?;
        if meta.name() != "visitor" {
            return Err(attr.span().error("Unexpected inner attribute"));
        }

        // FIXME: Use Darling's diagnostics support once it stabilizes
        // (see darling::Error::to_diagnostic)
        let mut conf =
            VisitorTraitConf::from_meta(&meta).map_err(|_| attr.span().error("Invalid syntax"))?;

        let name_string = conf.name.to_string();
        if self.names.contains(&name_string) {
            return Err(attr
                .span()
                .error(format!("Visitor `{}` defined more than once", name_string)));
        }
        if let (None, None) = (&conf.leave, &conf.enter) {
            let default_ident = proc_macro2::Ident::new("visit", proc_macro2::Span::call_site());
            conf.leave = Some(default_ident);
        }
        if let (Some(leave), Some(enter)) = (&conf.leave, &conf.enter) {
            if leave == enter {
                return Err(attr.span().error(format!(
                    "Same identifier `{}` used for both leave and enter",
                    leave.to_string()
                )));
            }
        }
        self.names.insert(name_string);
        Ok(conf)
    }
}

#[derive(Debug, FromMeta)]
pub struct VisitorTraitConf {
    #[darling(default)]
    pub enter: Option<proc_macro2::Ident>,
    #[darling(default)]
    pub leave: Option<proc_macro2::Ident>,
    #[darling(default)]
    pub public: bool,
    pub name: proc_macro2::Ident,
}

impl VisitorTraitConf {
    pub fn accept_trait_ident(&self) -> syn::Ident {
        let visitor_trait_string = self.name.to_string();
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
