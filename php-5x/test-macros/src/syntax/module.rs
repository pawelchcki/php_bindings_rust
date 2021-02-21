use syn::{parse, ItemFn, ItemMod};

pub enum InnerMacros {
    ModuleInit(ItemFn),
}

pub fn parse_inner_macros(module: &ItemMod) -> syn::Result<Vec<InnerMacros>> {
    let syn::ItemMod {
        attrs: _,
        vis: _,
        mod_token: _,
        ident,
        content,
        semi: _,
    } = module;

    let items = match content {
        Some((_, items)) => items,
        None => {
            return Ok( Vec::new() );
        }
    };
    let mut inner_macros: Vec<InnerMacros> = Vec::new();  
    for item in items {
        match item {
            syn::Item::Fn(f) => {
                inner_macros.append(&mut inner_macros_from_fn(f));
            }
            _ => {
                return Err(parse::Error::new(
                    ident.span(),
                    "unsupported module entry, refer to documentation about module declaration (if we had documentation :) )",
                ));
            }
        }
    }

    Ok( inner_macros )
}

fn inner_macros_from_fn(f: &ItemFn) -> Vec<InnerMacros> {
    let attributes = &f.attrs;
    let mut known_macros =  Vec::<InnerMacros>::new();
    for attr in attributes {
        let ident = match attr.style {
            syn::AttrStyle::Outer => {
                let segments = &attr.path.segments;
                if segments.len() == 1 {
                    segments.first().map(|p| p.ident.to_string())
                } else {
                    None
                }
            }
            syn::AttrStyle::Inner(_) => { None }
        };

        match ident {
            Some(p) => {
                if p == "module_init" {
                    known_macros.push(InnerMacros::ModuleInit(f.clone()));
                }
            }
            None => {}
        }
    }
    return known_macros;
}
