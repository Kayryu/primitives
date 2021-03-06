use crate::{U64, U128, U256, U512};

construct_fixed_hash!{ pub struct H32(4); }
construct_fixed_hash!{ pub struct H64(8); }
construct_fixed_hash!{ pub struct H128(16); }
construct_fixed_hash!{ pub struct H160(20); }
construct_fixed_hash!{ pub struct H256(32); }
construct_fixed_hash!{ pub struct H264(33); }
construct_fixed_hash!{ pub struct H512(64); }
construct_fixed_hash!{ pub struct H520(65); }


#[cfg(feature = "serialize")] impl_hash_serde!(H32, 4);
#[cfg(feature = "serialize")] impl_hash_serde!(H64, 8);
#[cfg(feature = "serialize")] impl_hash_serde!(H128, 16);
#[cfg(feature = "serialize")] impl_hash_serde!(H160, 20);
#[cfg(feature = "serialize")] impl_hash_serde!(H256, 32);
#[cfg(feature = "serialize")] impl_hash_serde!(H264, 33);
#[cfg(feature = "serialize")] impl_hash_serde!(H512, 64);
#[cfg(feature = "serialize")] impl_hash_serde!(H520, 65);

impl_uint_conversions!(H64, U64);
impl_uint_conversions!(H128, U128);
impl_uint_conversions!(H256, U256);
impl_uint_conversions!(H512, U512);

impl_fixed_hash_conversions!(H256, H160);

impl From<u64> for H160 {
	fn from(val: u64) -> Self {
		H160::from_low_u64_be(val)
	}
}

impl From<u64> for H256 {
	fn from(val: u64) -> Self {
		H256::from_low_u64_be(val)
	}
}

#[cfg(test)]
mod tests {
	use super::{H160, H256};
	use serde_json as ser;

	#[test]
	fn test_serialize_h160() {
		let tests = vec![
			(H160::from(0), "0x0000000000000000000000000000000000000000"),
			(H160::from(2), "0x0000000000000000000000000000000000000002"),
			(H160::from(15), "0x000000000000000000000000000000000000000f"),
			(H160::from(16), "0x0000000000000000000000000000000000000010"),
			(H160::from(1_000), "0x00000000000000000000000000000000000003e8"),
			(H160::from(100_000), "0x00000000000000000000000000000000000186a0"),
			(H160::from(u64::max_value()), "0x000000000000000000000000ffffffffffffffff"),
		];

		for (number, expected) in tests {
			assert_eq!(format!("{:?}", expected), ser::to_string_pretty(&number).unwrap());
			assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
		}
	}

	#[test]
	fn test_serialize_h256() {
		let tests = vec![
			(H256::from(0), "0x0000000000000000000000000000000000000000000000000000000000000000"),
			(H256::from(2), "0x0000000000000000000000000000000000000000000000000000000000000002"),
			(H256::from(15), "0x000000000000000000000000000000000000000000000000000000000000000f"),
			(H256::from(16), "0x0000000000000000000000000000000000000000000000000000000000000010"),
			(H256::from(1_000), "0x00000000000000000000000000000000000000000000000000000000000003e8"),
			(H256::from(100_000), "0x00000000000000000000000000000000000000000000000000000000000186a0"),
			(H256::from(u64::max_value()), "0x000000000000000000000000000000000000000000000000ffffffffffffffff"),
		];

		for (number, expected) in tests {
			assert_eq!(format!("{:?}", expected), ser::to_string_pretty(&number).unwrap());
			assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
		}
	}

	#[test]
	fn test_serialize_invalid() {
		assert!(ser::from_str::<H256>("\"0x000000000000000000000000000000000000000000000000000000000000000\"").unwrap_err().is_data());
		assert!(ser::from_str::<H256>("\"0x000000000000000000000000000000000000000000000000000000000000000g\"").unwrap_err().is_data());
		assert!(ser::from_str::<H256>("\"0x00000000000000000000000000000000000000000000000000000000000000000\"").unwrap_err().is_data());
		assert!(ser::from_str::<H256>("\"\"").unwrap_err().is_data());
		assert!(ser::from_str::<H256>("\"0\"").unwrap_err().is_data());
		assert!(ser::from_str::<H256>("\"10\"").unwrap_err().is_data());
	}
}
