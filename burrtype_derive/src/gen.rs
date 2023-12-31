mod parse;
mod attrs;
mod meta;

pub use meta::BurrMeta;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, Fields, FieldsNamed, FieldsUnnamed, Variant};

#[cfg(feature = "auto_register")]
pub fn auto_registration_fn(name: Ident) -> TokenStream {
    use inflector::Inflector;

    let fn_name = quote::format_ident!("burr_add_{}_type_registration", name.to_string().to_snake_case());
        quote! {
        #[burrtype::linkme::distributed_slice(burrtype::TYPES)]
        #[linkme(crate = burrtype::linkme)]
        #[doc(hidden)]
        fn #fn_name() -> burrtype::ir::IrItem {
            <#name as burrtype::ir::IrExt>::get_ir()
        }
    }
}

#[cfg(not(feature = "auto_register"))]
pub fn auto_registration_fn(_name: Ident) -> TokenStream {
    quote!()
}

pub fn named_struct_ir(
    attrs: Vec<Attribute>,
    name: Ident,
    fields: FieldsNamed,
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let module = attrs::container::burr_mod(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);
    let case = attrs::serde_rename_all(&attrs);

    // Collect fragments for each field that inserts the field's IR
    let field_impls = fields.named.iter()
        .map(|field| {
            let name = field.ident.clone().unwrap();
            match parse::named_field_attrs(field) {
                Ok(ir) => {
                    let ty = ir.ty.as_ref().map_or_else(|| &field.ty, |d| d);
                    let st = if ir.ignore {
                        quote!()
                    } else if ir.flatten {
                        quote! {fields.extend(<#ty as burrtype::ir::NamedStructExt>::fields());}
                    } else {
                        let (ty, optional) = parse::option(ty.clone());
                        let (ty, vec) = parse::vec(ty);
                        let field_docs = attrs::docs(&field.attrs);
                        let name = Ident::new_raw(&case.transform(&name.to_string()), name.span());
                        let ident = attrs::serde_rename(&field.attrs, &name);

                        quote! {
                            fields.push(burrtype::ir::IrNamedField {
                                ident: burrtype::syn::parse_quote!(#ident),
                                ty: burrtype::ir::IrType {
                                    id: std::any::TypeId::of::<#ty>(),
                                    path: burrtype::syn::parse_quote!(#ty),
                                    optional: #optional,
                                    array: #vec,
                                },
                                #field_docs
                            });
                        }
                    };
                    st
                }
                Err(err) => panic!("{}", err),
            }
        })
        .collect::<Vec<_>>();

    let field_map_frag = quote! {
        let mut fields = Vec::<burrtype::ir::IrNamedField>::new();
        #( #field_impls )*
        fields
    };

    let impl_ext_frag = quote! {
            impl burrtype::ir::NamedStructExt for #name {
                fn fields() -> Vec<burrtype::ir::IrNamedField> {
                    #field_map_frag
                }
            }
        };

    quote! {
        #impl_ext_frag

        impl burrtype::ir::IrExt for #name {
            fn get_ir() -> burrtype::ir::IrItem {
                let mut fields = Vec::<burrtype::ir::IrNamedField>::new();
                #( #field_impls )*

                burrtype::ir::IrNamedStruct {
                    ident: burrtype::syn::parse_quote!(#ident),
                    id: std::any::TypeId::of::<#name>(),
                    fields,
                    r#mod: #module,
                    #ir_docs

                }.into()
            }
        }
    }
}

pub fn tuple_struct_ir(
    attrs: Vec<Attribute>,
    name: Ident,
    fields: FieldsUnnamed
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let module = attrs::container::burr_mod(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);

    // Collect fragments for each field that describes the field's IR
    let field_ir = fields.unnamed.iter()
        .map(|field| {
            match parse::unnamed_field_attrs(field) {
                Ok(ir) => {
                    let ty = ir.ty.as_ref().map_or_else(|| &field.ty, |d| d);
                    let st = if ir.ignore {
                        quote!()
                    } else {
                        let field_docs = attrs::docs(&field.attrs);
                        let (ty, optional) = parse::option(ty.clone());
                        let (ty, vec) = parse::vec(ty);

                        if optional {
                            panic!("Option types unsupported for tuple structs");
                        }

                        quote! {
                            burrtype::ir::IrUnnamedField {
                                ty: burrtype::ir::IrType {
                                    id: std::any::TypeId::of::<#ty>(),
                                    path: burrtype::syn::parse_quote!(#ty),
                                    optional: #optional,
                                    array: #vec,
                                },
                                #field_docs
                            },
                        }
                    };
                    st
                }
                Err(err) => panic!("{}", err),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl burrtype::ir::IrExt for #name {
            fn get_ir() -> burrtype::ir::IrItem {
                burrtype::ir::IrTupleStruct {
                    ident: burrtype::syn::parse_quote!(#ident),
                    id: std::any::TypeId::of::<#name>(),
                    fields: vec![#(#field_ir)*],
                    r#mod: #module,
                    #ir_docs
                }.into()
            }
        }
    }
}

pub fn unit_struct_ir(
    attrs: Vec<Attribute>,
    name: Ident
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let module = attrs::container::burr_mod(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);

    quote! {
        impl burrtype::ir::IrExt for #name {
            fn get_ir() -> burrtype::ir::IrItem {
                burrtype::ir::IrUnitStruct {
                    ident: burrtype::syn::parse_quote!(#ident),
                    id: std::any::TypeId::of::<#name>(),
                    r#mod: #module,
                    #ir_docs
                }.into()
            }
        }
    }
}

pub fn enum_ir(
    attrs: Vec<Attribute>,
    name: Ident,
    data: DataEnum
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let module = attrs::container::burr_mod(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);
    let case = attrs::serde_rename_all(&attrs);
    let repr = attrs::container::serde_enum_repr(&attrs);

    // Collect fragments for each variant that describes the variant's IR
    let variant_frags = data.variants.into_iter().map(|var| {
        let Variant { attrs, ident, fields, .. } = var;

        if attrs::variant::serde_skip(&attrs) {
            return quote!()
        }

        if var.discriminant.is_some() {
            panic!("Enums with discriminants are unsupported");
        }

        let ident = Ident::new_raw(&case.transform(&ident.to_string()), ident.span());

        match fields {
            Fields::Named(inner) => enum_struct_variant_ir(attrs, ident, inner),
            Fields::Unnamed(inner) => enum_tuple_variant_ir(attrs, ident, inner),
            Fields::Unit => enum_unit_variant_ir(attrs, ident),
        }
    })
    .collect::<Vec<_>>();

    quote! {
        impl burrtype::ir::IrExt for #name {
            fn get_ir() -> burrtype::ir::IrItem {
                let mut variants = Vec::<burrtype::ir::IrEnumVariant>::new();
                #( #variant_frags )*

                burrtype::ir::IrEnum {
                    ident: burrtype::syn::parse_quote!(#ident),
                    id: std::any::TypeId::of::<#name>(),
                    variants,
                    repr: burrtype::ir::#repr,
                    r#mod: #module,
                    #ir_docs
                }.into()
            }
        }
    }
}

fn enum_struct_variant_ir(
    attrs: Vec<Attribute>,
    name: Ident,
    fields: FieldsNamed,
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);
    let case = attrs::serde_rename_all(&attrs);

    // Collect fragments for each field that inserts the field's IR
    let field_impls = fields.named.iter()
        .map(|field| {
            let name = field.ident.clone().unwrap();
            let ty = &field.ty;
            match parse::named_field_attrs(field) {
                Ok(ir) => {
                    let st = if ir.ignore {
                        quote!()
                    } else if ir.flatten {
                        quote! {fields.extend(<#ty as burrtype::ir::NamedStructExt>::fields());}
                    } else {
                        let (ty, optional) = parse::option(ty.clone());
                        let (ty, vec) = parse::vec(ty);
                        let field_docs = attrs::docs(&field.attrs);
                        let name = Ident::new_raw(&case.transform(&name.to_string()), name.span());
                        let ident = attrs::serde_rename(&field.attrs, &name);

                        quote! {
                            fields.push(burrtype::ir::IrNamedField {
                                ident: burrtype::syn::parse_quote!(#ident),
                                ty: burrtype::ir::IrType {
                                    id: std::any::TypeId::of::<#ty>(),
                                    path: burrtype::syn::parse_quote!(#ty),
                                    optional: #optional,
                                    array: #vec,
                                },
                                #field_docs
                            });
                        }
                    };
                    st
                }
                Err(err) => panic!("{}", err),
            }
        })
        .collect::<Vec<_>>();

    quote! {
        let mut fields = Vec::<burrtype::ir::IrNamedField>::new();
        #( #field_impls )*

        variants.push(burrtype::ir::IrEnumStructVariant {
            ident: burrtype::syn::parse_quote!(#ident),
            fields,
            #ir_docs
        }.into());
    }
}

fn enum_tuple_variant_ir(
    attrs: Vec<Attribute>,
    name: Ident,
    fields: FieldsUnnamed,
) -> TokenStream {
    // Collect fragments for each field that describes the field's IR
    let field_ir = fields.unnamed.iter()
        .map(|field| {
            let ty = &field.ty;

            match parse::unnamed_field_attrs(field) {
                Ok(ir) => {
                    let st = if ir.ignore {
                        quote!()
                    } else {
                        let field_docs = attrs::docs(&field.attrs);
                        let (ty, optional) = parse::option(ty.clone());
                        let (ty, vec) = parse::vec(ty);

                        if optional {
                            panic!("Option types unsupported for tuple variants");
                        }

                        quote! {
                            burrtype::ir::IrUnnamedField {
                                ty: burrtype::ir::IrType {
                                    id: std::any::TypeId::of::<#ty>(),
                                    path: burrtype::syn::parse_quote!(#ty),
                                    optional: #optional,
                                    array: #vec,
                                },
                                #field_docs
                            },
                        }
                    };
                    st
                }
                Err(err) => panic!("{}", err),
            }
        })
        .collect::<Vec<_>>();

    let ir_docs = attrs::docs(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);

    quote! {
        variants.push(burrtype::ir::IrEnumTupleVariant {
            ident: burrtype::syn::parse_quote!(#ident),
            fields: vec![#(#field_ir)*],
            #ir_docs
        }.into());
    }
}

fn enum_unit_variant_ir(
    attrs: Vec<Attribute>,
    name: Ident
) -> TokenStream {
    let ir_docs = attrs::docs(&attrs);
    let ident = attrs::serde_rename(&attrs, &name);

    quote! {
        variants.push(burrtype::ir::IrEnumUnitVariant {
            ident: burrtype::syn::parse_quote!(#ident),
            #ir_docs
        }.into());
    }
}

