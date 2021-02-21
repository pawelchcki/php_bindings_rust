pub mod php56;
pub mod php54;


#[derive(Debug)]
pub struct Args {
    pub version: Option<syn::Expr>,
    pub name: Option<syn::Expr>,
}

