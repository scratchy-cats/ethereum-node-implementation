#![allow(non_snake_case)]

pub mod encoding;
pub mod header;

// RLP prefix byte for 0-length string.
pub const EMPTY_STRING_CODE: u8 = 0x80; // = 128 (in decimal).

// RLP prefix byte for a 0-length array.
pub const EMPTY_LIST_CODE: u8 = 0xC0; // = 192 (in deciaml).

macro_rules! toBytesWithoutLeadingZeroes {
  ($x: expr, $bytes: ident) => {{
    $bytes = $x.to_be_bytes();
    &$bytes[($x.leading_zeros() / 8) as usize..]
  }};
}
pub(crate) use toBytesWithoutLeadingZeroes;
