use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

mod args;
use args::*;

#[proc_macro_attribute]
pub fn expect_op(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);

    expect_op_impl(args, input)
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn expect_op_for_axum_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = parse_macro_input!(args as Args);
    if args.crate_name_str != "expect_json" {
        panic!("expect_op does not support internal for axum test");
    }

    args.crate_name_str = "::axum_test::expect_json";
    args.crate_name = quote!(::axum_test::expect_json);

    expect_op_impl(args, input)
}

fn expect_op_impl(args: Args, input: TokenStream) -> TokenStream {
    let crate_name_str = args.crate_name_str;
    let crate_name = args.crate_name;

    let input_tokens: TokenStream2 = input.clone().into();
    let input_item = syn::parse_macro_input!(input as syn::Item);
    let struct_name = match input_item {
        syn::Item::Struct(item_struct) => item_struct.ident,
        syn::Item::Enum(item_enum) => item_enum.ident,
        _ => panic!("expect_op can only be used on structs or enums"),
    };
    let struct_name_str = args.display_name.unwrap_or_else(|| struct_name.to_string());
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
                #crate_name::__private::SerializeExpectOp::serialize(self, serializer)
            }
        }

        use #crate_name::__private::typetag;
        #[#crate_name::__private::typetag::serde]
        impl #crate_name::__private::ExpectOpSerialize for #struct_name {}

        impl #crate_name::__private::ExpectOpExt for #struct_name {
            fn name(&self) -> &'static str {
                #struct_name_str
            }
        }

        impl From<#struct_name> for #crate_name::__private::serde_json::Value {
            fn from(value: #struct_name) -> Self {
                #crate_name::__private::serde_json::to_value(&value).unwrap()
            }
        }
    };

    output.into()
}
