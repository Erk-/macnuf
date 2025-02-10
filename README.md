# macnuf

A very simple `#![no_std]` Rust library to get the manufacturer of a specific MAC address

## Getting Started

### Installation

If you want to use this library for one of your projects, you can install it like any other Rust crate

```bash
cargo add macnuf
```

### Example Usage

To get a manufacturer, you simply need to do the following

```rust
fn main() {
    match macnuf::lookup("00:18:23:ac:09:02".parse().unwrap()) {
        Some(manuf) => {
            println!("Manufacturer: {}", manuf)
        }
        None => {
            println!("No manufacturer found")
        }
    }
}
```

## License

This library is under the [MIT License](./LICENSE.md).

## Acknowledgement

This library is based upon [rsmanuf](https://github.com/kkrypt0nn/rsmanuf) by [Krypton](https://github.com/kkrypt0nn).
