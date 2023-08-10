
//! Autogenerated weights for `pallet_encointer_bazaar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// pallet
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_bazaar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_bazaar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_encointer_bazaar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_bazaar::WeightInfo for WeightInfo<T> {
	/// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	/// Proof: EncointerCommunities CommunityIdentifiers (max_values: Some(1), max_size: Some(90002), added: 90497, mode: MaxEncodedLen)
	/// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar BusinessRegistry (max_values: None, max_size: None, mode: Measured)
	fn create_business() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `91487`
		// Minimum execution time: 28_826_000 picoseconds.
		Weight::from_parts(31_307_000, 0)
			.saturating_add(Weight::from_parts(0, 91487))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar BusinessRegistry (max_values: None, max_size: None, mode: Measured)
	fn update_business() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3627`
		// Minimum execution time: 24_215_000 picoseconds.
		Weight::from_parts(26_309_000, 0)
			.saturating_add(Weight::from_parts(0, 3627))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar BusinessRegistry (max_values: None, max_size: None, mode: Measured)
	fn delete_business() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3627`
		// Minimum execution time: 27_141_000 picoseconds.
		Weight::from_parts(28_669_000, 0)
			.saturating_add(Weight::from_parts(0, 3627))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar BusinessRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: EncointerBazaar OfferingRegistry (r:0 w:1)
	/// Proof Skipped: EncointerBazaar OfferingRegistry (max_values: None, max_size: None, mode: Measured)
	fn create_offering() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3627`
		// Minimum execution time: 29_322_000 picoseconds.
		Weight::from_parts(30_740_000, 0)
			.saturating_add(Weight::from_parts(0, 3627))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: EncointerBazaar OfferingRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar OfferingRegistry (max_values: None, max_size: None, mode: Measured)
	fn update_offering() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3661`
		// Minimum execution time: 21_668_000 picoseconds.
		Weight::from_parts(22_897_000, 0)
			.saturating_add(Weight::from_parts(0, 3661))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerBazaar OfferingRegistry (r:1 w:1)
	/// Proof Skipped: EncointerBazaar OfferingRegistry (max_values: None, max_size: None, mode: Measured)
	fn delete_offering() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3661`
		// Minimum execution time: 19_727_000 picoseconds.
		Weight::from_parts(20_387_000, 0)
			.saturating_add(Weight::from_parts(0, 3661))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}