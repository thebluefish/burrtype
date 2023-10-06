use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{Expr, MetaList, Path, Token, token};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};

/// Like syn::Meta with an extra variant to support keywords in `name = value` attributes
pub enum BurrMeta {
    Path(Path),
    List(MetaList),
    KeywordValue(TypeValue),
}

impl ToTokens for BurrMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            BurrMeta::Path(inner) => inner.to_tokens(tokens),
            BurrMeta::List(inner) => inner.to_tokens(tokens),
            BurrMeta::KeywordValue(inner) => inner.to_tokens(tokens),
        }
    }
}

impl Parse for BurrMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(token::Paren) || input.peek2(token::Bracket) || input.peek2(token::Brace) {
            Ok(BurrMeta::List(input.parse::<MetaList>()?))
        }
        else if input.peek2(Token![=]) {
            input.parse().map(BurrMeta::KeywordValue)
        }
        else {
            Ok(BurrMeta::Path(input.call(Path::parse_mod_style)?))
        }
    }
}

pub struct TypeValue {
    pub path: Ident,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl ToTokens for TypeValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl Parse for TypeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TypeValue {
            path: Ident::parse_any(input)?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
