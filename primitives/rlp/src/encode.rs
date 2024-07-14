use super::{toBytesWithoutLeadingZeroes, EMPTY_STRING_CODE};
use crate::header::RLPEncodingHeader;

pub trait RLPEncodable {
  // RLP encodes the type and stores the output in the given buffer.
  fn rlpEncode(&self, buffer: &mut Vec<u8>);

  // Returns the byte length of the encoding output.
  fn rlpEncodingByteLen(&self) -> usize {
    let mut buffer: Vec<u8> = vec![];
    self.rlpEncode(&mut buffer);
    buffer.len()
  }
}

macro_rules! implRLPEncodableForUIntTypes {
  ($($uintType: ty),+ $(,)?) => {$(
		impl RLPEncodable for $uintType {
			#[inline]
			fn rlpEncode(&self, buffer: &mut Vec<u8>) {
				let x= *self;

				if x == 0 {
					buffer.push(EMPTY_STRING_CODE)
				}
				else if x < (EMPTY_STRING_CODE as $uintType) {
					buffer.push(x as u8)
				}
				else {
					let bytes;
					let bytesWithoutLeadingZeros= toBytesWithoutLeadingZeroes!(x, bytes);

					buffer.push(EMPTY_STRING_CODE + (bytesWithoutLeadingZeros.len() as u8));
					buffer.extend_from_slice(bytesWithoutLeadingZeros);
				}
			}

			#[inline]
			fn rlpEncodingByteLen(&self) -> usize {
				let x= *self;

				if x < (EMPTY_STRING_CODE as $uintType) {
					1
				}
				else {
					1 + (<$uintType>::BITS as usize / 8) - (x.leading_zeros() as usize / 8)
				}
			}
		}
	)+};
}
implRLPEncodableForUIntTypes!(u8, u16, u32, u64, u128, usize);

impl RLPEncodable for [u8] {
  fn rlpEncode(&self, buffer: &mut Vec<u8>) {
    if self.len() > 1 || self[0] > EMPTY_STRING_CODE {
      let rlpEncodingHeader = RLPEncodingHeader::new(false, self.len());
      rlpEncodingHeader.rlpEncode(buffer);
    }
    buffer.extend_from_slice(self);
  }

  fn rlpEncodingByteLen(&self) -> usize {
    let mut rlpEncodingByteLen = self.len();
    if self.len() > 1 || self[0] > EMPTY_STRING_CODE {
      let rlpEncodingHeader = RLPEncodingHeader::new(false, self.len());
      rlpEncodingByteLen += rlpEncodingHeader.rlpEncodingByteLen();
    }
    rlpEncodingByteLen
  }
}

impl RLPEncodable for str {
  #[inline]
  fn rlpEncode(&self, buffer: &mut Vec<u8>) {
    self.as_bytes().rlpEncode(buffer);
  }

  #[inline]
  fn rlpEncodingByteLen(&self) -> usize {
    self.as_bytes().rlpEncodingByteLen()
  }
}

impl RLPEncodable for bool {
  #[inline]
  fn rlpEncode(&self, buffer: &mut Vec<u8>) {
    buffer.push(if *self { 1 } else { EMPTY_STRING_CODE });
  }

  #[cfg_attr(any(), rustfmt::skip)]
  #[inline]
  fn rlpEncodingByteLen(&self) -> usize { 1 }
}

impl<T: RLPEncodable> RLPEncodable for Vec<T> {
  #[inline]
  fn rlpEncode(&self, buffer: &mut Vec<u8>) {
    rlpEncodeList(self, buffer);
  }

  #[inline]
  fn rlpEncodingByteLen(&self) -> usize {
    getRLPEncodingByteLenForList(self)
  }
}

#[inline]
fn getRLPEncodingHeaderForList<T: RLPEncodable>(list: &[T]) -> RLPEncodingHeader {
  RLPEncodingHeader::new(
    true,
    list.iter().map(|item| item.rlpEncodingByteLen()).sum(),
  )
}

#[inline]
fn rlpEncodeList<T: RLPEncodable>(list: &[T], buffer: &mut Vec<u8>) {
  getRLPEncodingHeaderForList(list).rlpEncode(buffer);
  list.iter().for_each(|value| value.rlpEncode(buffer));
}

#[inline]
fn getRLPEncodingByteLenForList<T: RLPEncodable>(list: &[T]) -> usize {
  let rlpEncodingHeader = getRLPEncodingHeaderForList(list);
  rlpEncodingHeader.rlpEncodingByteLen() + rlpEncodingHeader.payloadByteLen()
}

#[inline]
pub fn getRLPEncodingHeaderByteLenForPayloadByteLen(payloadByteLen: usize) -> usize {
  if payloadByteLen < 56 {
    1
  }
  else {
    1 + (payloadByteLen / 8) - (payloadByteLen.leading_zeros() as usize / 8)
  }
}
