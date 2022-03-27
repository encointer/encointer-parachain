
//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Collective Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Voting (r:100 w:100)
	// Storage: Collective Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 206_000
			.saturating_add((19_611_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 206_000
			.saturating_add((2_865_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 206_000
			.saturating_add((22_526_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Collective Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(28_352_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((106_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(34_024_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((199_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalCount (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(35_045_000 as Weight)
			// Standard Error: 0
			.saturating_add((23_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 3_000
			.saturating_add((188_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 3_000
			.saturating_add((365_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(54_429_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((437_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(69_509_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((54_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 8_000
			.saturating_add((462_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	fn close_early_approved(_b: u32, m: u32, p: u32, ) -> Weight {
		(90_585_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((109_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 6_000
			.saturating_add((367_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(59_699_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((281_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 8_000
			.saturating_add((258_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(77_812_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 4_000
			.saturating_add((332_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 4_000
			.saturating_add((263_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(34_862_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((322_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
