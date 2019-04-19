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
	println!("{:?}", my_bloom);
}
