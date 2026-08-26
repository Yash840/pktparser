# pktparser

A simple packet parsing library written in Rust.

`pktparser` is designed to parse raw network packets layer by layer, turning bytes into useful, structured representations.

## Supported Protocols

Currently:

* Ethernet

More protocols will be added as the project grows, including IP, TCP, UDP, HTTP, and others.

## Example

```rust
use pktparser::ether::Ether;

fn main() {
    let packet: &[u8] = &[
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x08, 0x00,
    ];

    let frame = Ether::parse(packet).unwrap();

    println!("{}", frame);
}
```

## Goals

* Keep packet parsing simple and explicit
* Avoid unnecessary data copying
* Provide useful error handling
* Build protocol support incrementally

## Status

**Early development**

`pktparser` is currently a learning-focused project and is actively evolving.

## License

Licensed under the MIT License.
