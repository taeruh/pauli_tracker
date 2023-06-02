use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Data,
    DataEnum,
    DeriveInput,
    Error,
    Expr,
    ExprLit,
    Ident,
    Lit,
    LitStr,
    Meta,
    Token,
};

macro_rules! unwrap_or_compile_error {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => ok,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}

#[proc_macro_derive(ArgDispatch, attributes(arg_dispatch))]
pub fn arg_dispatch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, variants, serialized_variants) = unwrap_or_compile_error!(parse_input(&input));

    fn lit_str_from_meta(meta: &Meta) -> syn::Result<&LitStr> {
        match meta.require_name_value()?.value {
            Expr::Lit(ExprLit { lit: Lit::Str(ref s), .. }) => Ok(s),
            _ => {
                let value = &meta.require_name_value().unwrap().value;
                Err(Error::new(
                    meta.path().get_ident().unwrap().span(),
                    format!("{:?} is not a string literal", quote!(#value)),
                ))
            }
        }
    }

    let mut dispatcher = quote! {};

    for attr in &input.attrs {
        if !attr.path().is_ident("arg_dispatch") {
            continue;
        }
        for arg in attr
            .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap()
            .iter()
        {
            if arg.path().is_ident("module") {
                let lit = match lit_str_from_meta(arg) {
                    Ok(lit) => lit,
                    Err(err) => return err.to_compile_error().into(),
                };
                let ident = Ident::new(lit.value().as_str(), lit.span());
                dispatcher = quote! { #ident:: };
            }
        }
    }

    quote! {
        impl #name {
            fn dispatch(self) {
                match self {
                    #(#name::#variants => #dispatcher #serialized_variants(),)*
                    _ => (),
                }
            }
        }
    }
    .into()
}

#[allow(clippy::needless_lifetimes)] // bug?
fn parse_input<'a>(
    input: &'a DeriveInput,
) -> syn::Result<(&Ident, Vec<&Ident>, impl Iterator<Item = Ident> + 'a)> {
    let data = unwrap_enum(input)?;

    let name = &input.ident;
    let variants = data.variants.iter().map(|e| (&e.ident)).collect::<Vec<_>>();
    let serialized_variants = variants
        .clone()
        .into_iter()
        .map(|e| Ident::new(snake_case(e.to_string()).as_str(), e.span()));

    Ok((name, variants, serialized_variants))
}

fn unwrap_enum(input: &DeriveInput) -> syn::Result<&DataEnum> {
    if let Data::Enum(ref data) = input.data {
        Ok(data)
    } else {
        Err(Error::new(input.span(), "need enum"))
    }
}

fn snake_case(s: String) -> String {
    let mut res = String::new();
    for (i, c) in s.char_indices() {
        if i > 0 && c.is_uppercase() {
            res.push('_');
        }
        res.push(c.to_ascii_lowercase());
    }
    res
}
