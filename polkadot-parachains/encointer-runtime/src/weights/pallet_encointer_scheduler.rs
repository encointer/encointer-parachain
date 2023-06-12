
//! Autogenerated weights for `pallet_encointer_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_encointer_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: EncointerScheduler CurrentPhase (r:1 w:1)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:1)
	// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:1)
	// Storage: EncointerScheduler PhaseDurations (r:3 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: EncointerCeremonies ReputationLifetime (r:1 w:0)
	// Storage: EncointerCeremonies InactivityTimeout (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	fn next_phase() -> Weight {
		Weight::from_parts(83_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:1)
	fn push_by_one_day() -> Weight {
		Weight::from_parts(24_000_000, 0)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerScheduler PhaseDurations (r:0 w:1)
	fn set_phase_duration() -> Weight {
		Weight::from_parts(3_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerScheduler NextPhaseTimestamp (r:0 w:1)
	fn set_next_phase_timestamp() -> Weight {
		Weight::from_parts(2_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
