use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Ident, Lit};

#[proc_macro_attribute]
pub fn url_suffix(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as Lit);

    let struct_name = input.ident.clone();
    let suffix = if let Lit::Str(lit_str) = attr {
        lit_str.value()
    } else {
        panic!("The attribute value must be a string");
    };

    let expanded = quote! {
        #input

        impl UrlSuffix for #struct_name {
            fn url_suffix(&self) -> String {
                #suffix.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ApiDataDerive)]
pub fn api_data_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = &ast.data {
        variants
    } else {
        panic!("ApiData can only be derived for enums");
    };

    let variant_names: Vec<&Ident> = variants.iter().map(|variant| &variant.ident).collect();

    let gen = quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #(
                        #name::#variant_names(inner) => inner.serialize(serializer),
                    )*
                }
            }
        }

        impl UrlSuffix for #name {
            fn url_suffix(&self) -> String {
                match &self {
                    #(
                        #name::#variant_names(inner) => inner.url_suffix(),
                    )*
                }
            }
        }
    };
    gen.into()
}
