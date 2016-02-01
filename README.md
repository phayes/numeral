# Numeral
Rust library providing the written english form of a number.

[Documentation](https://letheed.github.io/numeral/numeral/index.html)

### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
numeral = "^0.1"
```
and this to your crate root:
```rust
extern crate numeral;
```
### Example
```rust
use numeral::Numeral;

let n = 127;
println!("{} is written: {}", n, n.ordinal());
```