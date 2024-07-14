#![allow(non_snake_case)]

use rlp::encoding::RLPEncodable;

#[test]
fn encode() {
  #[derive(Debug, rlp_macros::RLPEncodable)]
  struct ContainerState {
    id: String,
    pid: u64,
  }

  let containerState = ContainerState {
    id: String::from("redis"),
    pid: 0,
  };

  let mut buffer: Vec<u8> = vec![];
  containerState.rlpEncode(&mut buffer);

  println!(
    "RLP encoding length for containerState {:?} : {}",
    containerState,
    containerState.rlpEncodingByteLen()
  );
}
