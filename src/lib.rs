use proc_macro::TokenStream;
use proc_macro::{self};
use quote::quote;
use syn::{self, spanned::Spanned};

#[proc_macro_derive(AllVariants)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = match syn::parse(input) {
        Ok(input) => input,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let variants = match input.data {
        syn::Data::Enum(data_enum) => {
            let mut identifiers = Vec::with_capacity(data_enum.variants.len());
            for variant in data_enum.variants.into_iter() {
                match variant.fields {
                    syn::Fields::Named(named) => {
                        return error_only_unit_fields(named.span());
                    }
                    syn::Fields::Unnamed(unnamed) => {
                        return error_only_unit_fields(unnamed.span());
                    }
                    syn::Fields::Unit => identifiers.push(variant.ident),
                }
            }
            identifiers
        }
        syn::Data::Struct(data_struct) => {
            return error_all_variants_only_for_enums(data_struct.struct_token);
        }
        syn::Data::Union(data_union) => {
            return error_all_variants_only_for_enums(data_union.union_token);
        }
    };
    let enum_name = input.ident;

    let expanded = quote! {
        impl #enum_name {
            #[doc="List all variants of this enumeration in order of declaration"]
            pub fn all_variants() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants),* ]
            }
        }
    };
    expanded.into()
}

fn error_all_variants_only_for_enums(spanned: impl Spanned) -> TokenStream {
    syn::Error::new(spanned.span(), "AllVariants can only be derived on enums")
        .into_compile_error()
        .into()
}

fn error_only_unit_fields(spanned: impl Spanned) -> TokenStream {
    syn::Error::new(
        spanned.span(),
        "AllVariants can only be derived on enums with only unit (primitive) variants",
    )
    .into_compile_error()
    .into()
}
