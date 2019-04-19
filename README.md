# Ethereum primitives

[![Build Status](https://travis-ci.org/paritytech/primitives.svg?branch=master)](https://travis-ci.org/paritytech/primitives)

Fixed-sized integer arithmetic (ethereum-types) and bloom filter (ethbloom)

To add this crate to your project, add the following in `Cargo.toml`

```toml
[dependencies]
primitives = { git = "https://github.com/Kayryu/primitives.git" , branch="develop" }
```

A basic example how to use this crate:

```rust
extern crate primitives;

use primitives::{U256, Bloom, BloomInput};

fn main() {
	let mut val: U256 = 1023.into();
	for _ in 0..200 { val = val * 2u32 }
	assert_eq!(
		&format!("{}", val),
		"1643897619276947051879427220465009342380213662639797070513307648"
	);

	let address = [0_u8; 32];
	let mut my_bloom = Bloom::default();
	assert!(!my_bloom.contains_input(BloomInput::Raw(&address)));
	my_bloom.accrue(BloomInput::Raw(&address));
}

```

### `no_std` crates

This crate has a feature, `std`, that is enabled by default. To use this crate
in a `no_std` context, add the following to your `Cargo.toml`:

```toml
[dependencies]
primitives = { version = "0.1.0", default-features = false }
```
