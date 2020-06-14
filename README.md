# Single digit Reverse Polish Notation 

[![Build Status](https://travis-ci.org/foresta/single-digit-rpn.svg?branch=master)](https://travis-ci.org/foresta/single-digit-rpn)

This package is a single digit RPN calculator.
Only single digit RPN is supported:

```
123++ => 1 + (2 + 3)
4 5 3 - * => 4 * (5 - 3)
7 8*9- => 7 * 8 - 9
```
# Usage

```rust
extern crate single_digit_rpn;
use single_digit_rpn::rpn;

let result: f64 = rpn("123++").unwrap();
assert_eq!(result, 6.0);
```

# Examples

You can find example code in the [examples directory](./examples).

To execute it, do the folloing:

```
cargo run --example cli
```
