pub mod php56;

#[derive(Debug)]
pub struct Args {
    pub version: Option<syn::Expr>,
    pub name: Option<syn::Expr>,
}
