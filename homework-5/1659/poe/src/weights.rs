
#![allow(unused_parens)]
#![allow(unused_imports)]
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;
pub trait WeightInfo {
	fn do_something() -> Weight;
	fn cause_error() -> Weight;
}
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn do_something() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn cause_error() -> Weight {
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn do_something() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_000_000 picoseconds.
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn cause_error() -> Weight {
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
