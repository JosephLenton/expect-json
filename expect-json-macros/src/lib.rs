use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::format_ident;
use proc_macro::Ident;

#[proc_macro_attribute]
pub fn expect_op(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Option<syn::Ident>);
    let crate_name = match args {
        None => format_ident!("expect_json"),
        Some(crate_name) => {
            if crate_name != "internal" {
                panic!("expect_op can only be used with `internal`");
            }

            format_ident!("crate")
        },
    };

    let input_tokens: TokenStream2 = input.clone().into();
    let input_item = syn::parse_macro_input!(input as syn::Item);
    let struct_name = match input_item {
        syn::Item::Struct(item_struct) => item_struct.ident,
        syn::Item::Enum(item_enum) => item_enum.ident,
        _ => panic!("expect_op can only be used on structs or enums"),
    };

    let output = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(crate = "crate::__private::serde_trampoline")]
        #input_tokens

        impl ::serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                #crate_name::SerializeExpectOp::serialize(self, serializer)
            }
        }

        #[typetag::serde]
        impl #crate_name::ExpectOpSerialize for #struct_name {}
    };

    output.into()
}
