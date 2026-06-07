use proc_macro2::Span;
use syn::Ident;

#[cfg(feature = "find-crate")]
pub fn find_crate_name() -> Ident {
    use find_crate::Error;

    match find_crate::find_crate(|name| name == "palette") {
        Ok(package) => Ident::new(&package.name, Span::call_site()),
        Err(Error::NotFound) => Ident::new("palette", Span::call_site()),
        Err(error) => panic!("error when trying to find the name of the `palette` crate: {error}"),
    }
}

#[cfg(not(feature = "find-crate"))]
pub fn find_crate_name() -> Ident {
    Ident::new("palette", Span::call_site())
}
