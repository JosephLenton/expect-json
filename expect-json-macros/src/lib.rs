use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;

#[proc_macro_attribute]
pub fn expect_op(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Option<syn::Ident>);
    let crate_name_str = match args {
        None => "expect_json",
        Some(crate_name) => {
            if crate_name != "internal" {
                panic!("expect_op can only be used with `internal`");
            }

            "crate"
        }
    };
    let crate_name = format_ident!("{crate_name_str}");

    let input_tokens: TokenStream2 = input.clone().into();
    let input_item = syn::parse_macro_input!(input as syn::Item);
    let struct_name = match input_item {
        syn::Item::Struct(item_struct) => item_struct.ident,
        syn::Item::Enum(item_enum) => item_enum.ident,
        _ => panic!("expect_op can only be used on structs or enums"),
    };
    let struct_name_str = struct_name.to_string();
    let serde_trampoline_path = format!("{crate_name_str}::__private::serde_trampoline");

    let output = quote! {
        #[derive(#crate_name::__private::serde::Serialize, #crate_name::__private::serde::Deserialize)]
        #[serde(crate = #serde_trampoline_path)]
        #input_tokens

        impl #crate_name::__private::serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: #crate_name::__private::serde::Serializer,
            {
                #crate_name::SerializeExpectOp::serialize(self, serializer)
            }
        }

        use #crate_name::__private::typetag;
        #[#crate_name::__private::typetag::serde]
        impl #crate_name::ExpectOpSerialize for #struct_name {}

        impl #crate_name::ExpectOpExt for #struct_name {
            fn name(&self) -> &'static str {
                #struct_name_str
            }
        }
    };

    output.into()
}
