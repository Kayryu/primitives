extern crate primitives;

use primitives::*;

#[no_mangle]
pub extern "C" fn u256_zero() -> U256 { U256::zero() }
