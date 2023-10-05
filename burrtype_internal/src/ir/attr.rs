use inflector::Inflector;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, TokenStreamExt, ToTokens};
use syn::spanned::Spanned;
use syn::Token;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CaseConvention {
    /// Leave items untouched
    Default,
    /// lowercase
    Lowercase,
    /// UPPERCASE
    Uppercase,
    /// PascalCase
    PascalCase,
    /// camelCase
    CamelCase,
    /// snake_case
    SnakeCase,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnakeCase,
    /// kebab-case
    KebabCase,
    /// SCREAMING-KEBAB-CASE
    ScreamingKebabCase,
}

impl CaseConvention {
    pub fn parse(case: &str) -> Self {
        match case {
            "lowercase" => CaseConvention::Lowercase,
            "UPPERCASE" => CaseConvention::Uppercase,
            "PascalCase" => CaseConvention::PascalCase,
            "camelCase" => CaseConvention::CamelCase,
            "snake_case" => CaseConvention::SnakeCase,
            "SCREAMING_SNAKE_CASE" => CaseConvention::ScreamingSnakeCase,
            "kebab-case" => CaseConvention::KebabCase,
            "SCREAMING-KEBAB-CASE" => CaseConvention::ScreamingKebabCase,
            _ => CaseConvention::Default,
        }
    }

    pub fn transform(&self, input: &str) -> String {
        match self {
            CaseConvention::Default => input.to_string(),
            CaseConvention::Lowercase => input.to_lowercase(),
            CaseConvention::Uppercase => input.to_uppercase(),
            CaseConvention::PascalCase => input.to_pascal_case(),
            CaseConvention::CamelCase => input.to_camel_case(),
            CaseConvention::SnakeCase => input.to_snake_case(),
            CaseConvention::ScreamingSnakeCase => input.to_screaming_snake_case(),
            CaseConvention::KebabCase => input.to_kebab_case(),
            CaseConvention::ScreamingKebabCase => input.to_kebab_case().to_uppercase(),
        }
    }
}

impl ToTokens for CaseConvention {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name: Ident = Ident::new("CaseConvention", Span::call_site());
        let case: Ident = Ident::new(match *self {
            CaseConvention::Default => "Default",
            CaseConvention::Lowercase => "Lowercase",
            CaseConvention::Uppercase => "Uppercase",
            CaseConvention::PascalCase => "PascalCase",
            CaseConvention::CamelCase => "CamelCase",
            CaseConvention::SnakeCase => "SnakeCase",
            CaseConvention::ScreamingSnakeCase => "ScreamingSnakeCase",
            CaseConvention::KebabCase => "KebabCase",
            CaseConvention::ScreamingKebabCase => "ScreamingKebabCase",
        }, Span::call_site());
        tokens.extend(quote! {
            #name :: #case
        })
    }
}
