# RLP (Recursive Length Prefix) Serialization Derive Macro

## Example

```rust
use rlp::encode::RLPEncodable;

#[derive(rlp_macros::RLPEncodable)]
struct ContainerState {
	id: String,
	pid: i64,
}

let containerState = ContainerState {
	id: String::from("redis"),
	pid: 611,
};

let mut buffer: Vec<u8> = vec![];
containerState.rlpEncode(&mut buffer);

println!(
	"RLP encoding length for containerState {:?} : {}",
	containerState,
	containerState.rlpEncodingByteLen()
);
```

## REFERENCES

- [RECURSIVE-LENGTH PREFIX (RLP) SERIALIZATION](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/)
- [RLP implementation by alloy-rs](https://github.com/alloy-rs/rlp)
- [A Comprehensive Guide to RLP Encoding in Ethereum](https://medium.com/@markodayansa/a-comprehensive-guide-to-rlp-encoding-in-ethereum-6bd75c126de0)
