use quote::quote;
use syn::{Attribute, DataStruct, DeriveInput, Error, Meta, Result};

pub(crate) fn parseStruct<'a>(
  ast: &'a DeriveInput,
  deriveAttribute: &str,
) -> Result<&'a DataStruct> {
  match &ast.data {
    syn::Data::Struct(s) => Ok(s),

    _ => Err(Error::new_spanned(
      ast,
      format!("#[derive({deriveAttribute})] can only be used with structs"),
    )),
  }
}

pub(crate) fn attributesIncludeMeta(attributes: &[Attribute], targetMeta: &str) -> bool {
  for attribute in attributes.iter() {
    if !attribute.path().is_ident("rlp") {
      continue;
    }

    if let Meta::List(metaList) = &attribute.meta {
      let mut targetMetaFound = false;

      let _ = metaList.parse_nested_meta(|meta| {
        targetMetaFound = meta.path.is_ident(targetMeta);
        Ok(())
      });

      if targetMetaFound {
        return true;
      }
    }
  }

  false
}

pub(crate) fn getStructFieldName(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
  if let Some(ident) = &field.ident {
    quote! { #ident }
  }
  else {
    let index = syn::Index::from(index);
    quote! { #index }
  }
}
