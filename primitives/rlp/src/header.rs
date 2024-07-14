use crate::{
  encoding::{getRLPEncodingHeaderByteLenForPayloadByteLen, RLPEncodable},
  toBytesWithoutLeadingZeroes, EMPTY_LIST_CODE, EMPTY_STRING_CODE,
};
use derive_getters::Getters;
use derive_more::Constructor;

#[derive(Constructor, Getters)]
pub struct RLPEncodingHeader {
  list: bool, // Vecs and Structs are considered as lists.
  payloadByteLen: usize,
}

impl RLPEncodable for RLPEncodingHeader {
  #[inline]
  fn rlpEncode(&self, buffer: &mut Vec<u8>) {
    if self.payloadByteLen < 56 {
      let startingByteBaseValue = if self.list { EMPTY_LIST_CODE } else { EMPTY_STRING_CODE };
      buffer.push(startingByteBaseValue + self.payloadByteLen as u8);
    }
    else {
      let byteLenAsBytes;
      let byteLenAsBytesWithoutLeadingZeros =
        toBytesWithoutLeadingZeroes!(self.payloadByteLen, byteLenAsBytes);

      let startingByteBaseValue = if self.list { 0xF7 } else { 0xB7 };
      buffer.push(startingByteBaseValue + byteLenAsBytesWithoutLeadingZeros.len() as u8);
      buffer.extend_from_slice(byteLenAsBytesWithoutLeadingZeros);
    }
  }

  #[inline]
  fn rlpEncodingByteLen(&self) -> usize {
    getRLPEncodingHeaderByteLenForPayloadByteLen(self.payloadByteLen)
  }
}
