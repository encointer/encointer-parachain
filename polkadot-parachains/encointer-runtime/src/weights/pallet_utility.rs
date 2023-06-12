
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-fresh"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=encointer-rococo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(54_338_000, 0)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(8_410_000_u64).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		Weight::from_ref_time(6_000_000, 0)
	}
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(216_369_000, 0)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(7_644_000_u64).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(23_000_000, 0)
	}

	fn force_batch(_c: u32) -> Weight {
		// Todo: dummy weight need to rerun benchmarks
		Weight::from_ref_time(23_000_000, 0)
	}
}
