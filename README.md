# Numeral

Rust library providing the written english form of a number.


## Usage

``` rust
extern crate numeral;

use numeral::Ordinal;

let n = 127;
println!("{} is written: {}", n, n.ordinal());
```
