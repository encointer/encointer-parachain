
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("launch-rococo"), DB CACHE: 128

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=launch-rococo
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/launch-runtime/src/weights/pallet_utility.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		(116_668_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((6_229_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(4_000_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(9_479_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((7_186_000 as Weight).saturating_mul(c as Weight))
	}
}