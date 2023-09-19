use proc_macro2::Ident;
use syn::{Attribute, Path, Item};

pub trait SynIdent {
    /// Checks whether the item matches the ident regardless of path
    fn is_ident<I: ?Sized>(&self, ident: &I) -> bool where Ident: PartialEq<I>;
    /// Gets the ident portion of this item
    fn get_ident(&self) -> Option<&Ident>;
}

impl SynIdent for Path {
    fn is_ident<I: ?Sized>(&self, ident: &I) -> bool
        where Ident: PartialEq<I>
    {
        if let Some(segment) = self.segments.last() {
            return &segment.ident == ident
        }
        false
    }

    fn get_ident(&self) -> Option<&Ident> {
        if let Some(segment) = self.segments.last() {
            return Some(&segment.ident)
        }
        None
    }
}

impl SynIdent for Attribute {
    fn is_ident<I: ?Sized>(&self, ident: &I) -> bool
        where Ident: PartialEq<I>
    {
        if let Some(segment) = self.path().segments.last() {
            return &segment.ident == ident
        }
        false
    }

    fn get_ident(&self) ->  Option<&Ident> {
        if let Some(segment) = self.path().segments.last() {
            return Some(&segment.ident)
        }
        None
    }
}

pub trait SynItemExt {
    fn get_ident<'a>(&'a self) -> Option<&'a Ident>;
    /// Gets the item's attributes if it has any
    fn get_attrs<'a>(&'a self) -> Option<&'a Vec<Attribute>>;
    fn has_attr<I: ?Sized>(&self, ident: &I) -> bool where Ident: PartialEq<I>;
    fn get_attr<'a, I: ?Sized>(&'a self, ident: &I) -> Option<&'a Attribute> where Ident: PartialEq<I>;
}

impl SynItemExt for Item {
    fn get_ident<'a>(&'a self) -> Option<&'a Ident> {
        match self {
            Item::Const(inner) => Some(&inner.ident),
            Item::Struct(inner) => Some(&inner.ident),
            Item::Enum(inner) => Some(&inner.ident),
            Item::Union(inner) => Some(&inner.ident),
            Item::Fn(inner) => Some(&inner.sig.ident),
            Item::Trait(inner) => Some(&inner.ident),
            Item::Type(inner) => Some(&inner.ident),
            Item::Mod(inner) => Some(&inner.ident),
            Item::ExternCrate(inner) => Some(&inner.ident),
            Item::Static(inner) => Some(&inner.ident),
            Item::TraitAlias(inner) => Some(&inner.ident),
            _ => None,
        }
    }
    fn get_attrs<'a>(&'a self) -> Option<&'a Vec<Attribute>> {
        match self {
            Item::Const(inner) => Some(&inner.attrs),
            Item::Struct(inner) => Some(&inner.attrs),
            Item::Enum(inner) => Some(&inner.attrs),
            Item::Union(inner) => Some(&inner.attrs),
            Item::Macro(inner) => Some(&inner.attrs),
            Item::Fn(inner) => Some(&inner.attrs),
            Item::Impl(inner) => Some(&inner.attrs),
            Item::Trait(inner) => Some(&inner.attrs),
            Item::Type(inner) => Some(&inner.attrs),
            Item::Use(inner) => Some(&inner.attrs),
            Item::Mod(inner) => Some(&inner.attrs),
            Item::ExternCrate(inner) => Some(&inner.attrs),
            Item::ForeignMod(inner) => Some(&inner.attrs),
            Item::Static(inner) => Some(&inner.attrs),
            Item::TraitAlias(inner) => Some(&inner.attrs),
            _ => None,
        }
    }

    /// Returns true if the item has an attribute with the given ident, ignoring its path if any
    fn has_attr<I: ?Sized>(&self, ident: &I) -> bool
        where Ident: PartialEq<I>
    {
        if let Some(attrs) = self.clone().get_attrs() {
            for attr in attrs {
                if attr.is_ident(ident) {
                    return true
                }
            }
        }

        false
    }

    /// Returns true if the item has an attribute with the given ident, ignoring its path if any
    fn get_attr<'a, I: ?Sized>(&'a self, ident: &I) -> Option<&'a Attribute>
        where Ident: PartialEq<I>
    {
        if let Some(attrs) = self.get_attrs() {
            for attr in attrs {
                if attr.is_ident(ident) {
                    return Some(attr)
                }
            }
        }

        None
    }
}
