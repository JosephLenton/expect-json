use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use syn::LitStr;
use syn::Result as SynResult;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token;

mod keyword {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(internal);
}

pub struct Args {
    pub crate_name_str: &'static str,
    pub crate_name: TokenStream2,
    pub display_name: Option<String>,
}

impl Args {
    pub fn new(is_internal: bool, display_name: Option<String>) -> Self {
        let crate_name_str = if is_internal { "crate" } else { "expect_json" };
        let crate_name_ident = format_ident!("{crate_name_str}");
        let crate_name = quote!(#crate_name_ident);

        Self {
            crate_name_str,
            crate_name,
            display_name,
        }
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut maybe_display_name = None;
        let mut maybe_is_internal = false;

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            // for #[expect_op(internal)]
            if lookahead.peek(keyword::internal) {
                input.parse::<keyword::internal>()?;
                maybe_is_internal = true;

                if input.peek(token::Comma) {
                    input.parse::<token::Comma>()?;
                }

            // for #[expect_op(name = "...")]
            } else if lookahead.peek(keyword::name) {
                let display_name = input.parse::<DisplayName>()?;
                maybe_display_name = Some(display_name.name);

                if input.peek(token::Comma) {
                    input.parse::<token::Comma>()?;
                }
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(Self::new(maybe_is_internal, maybe_display_name))
    }
}

struct DisplayName {
    name: String,
}

impl Parse for DisplayName {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.parse::<keyword::name>()?;
        input.parse::<token::Eq>()?;
        let name = input.parse::<LitStr>()?.value();

        Ok(Self { name })
    }
}
