#![allow(non_snake_case)]

mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput, Field, Result};
use utils::{attributesIncludeMeta, parseStruct};

#[proc_macro_derive(RLPEncodable, attributes(rlp))]
pub fn rlpEncodable(input: TokenStream) -> TokenStream {
  parse(input)
    .and_then(|ast| implRLPEncodable(&ast))
    .unwrap_or_else(|error| error.to_compile_error())
    .into()
}

fn implRLPEncodable(ast: &DeriveInput) -> Result<proc_macro2::TokenStream> {
  let structName = &ast.ident;

  let structFields: Vec<(usize, &Field)> = parseStruct(ast, "RLPEncodable")?
    .fields
    .iter()
    .enumerate()
    .filter(|(_, field)| !attributesIncludeMeta(&field.attrs, "skip"))
    .collect();

  let mut rlpEncodeExpressions = Vec::with_capacity(structFields.len());
  let mut rlpEncodingByteLenExpressions = Vec::with_capacity(structFields.len());

  let mut structFieldsIter = structFields.iter();
  while let Some((i, structField)) = structFieldsIter.next() {
    let structFieldName = getStructFieldName(*i, structField);

    rlpEncodeExpressions.push(quote! {
      self.#structFieldName.rlpEncode(buffer);
    });

    rlpEncodingByteLenExpressions.push(quote! {
      self.#structFieldName.rlpEncodingByteLen()
    })
  }

  Ok(quote! {
    impl rlp::encode::RLPEncodable for #structName {
      #[inline]
      fn rlpEncode(&self, buffer: &mut Vec<u8>) {
        rlp::header::RLPEncodingHeader::new(true, self.rlpEncodingPayloadByteLen())
          .rlpEncode(buffer);
        #(#rlpEncodeExpressions)*
      }

      #[inline]
      fn rlpEncodingByteLen(&self) -> usize {
        let payloadByteLen = self.rlpEncodingPayloadByteLen();
        rlp::encode::getRLPEncodingHeaderByteLenForPayloadByteLen(payloadByteLen) + payloadByteLen
      }
    }

    impl #structName {
      fn rlpEncodingPayloadByteLen(&self) -> usize {
        0 #(+ #rlpEncodingByteLenExpressions)*
      }
    }
  })
}

fn getStructFieldName(index: usize, field: &syn::Field) -> proc_macro2::TokenStream {
  if let Some(ident) = &field.ident {
    quote! { #ident }
  }
  else {
    let index = syn::Index::from(index);
    quote! { #index }
  }
}
