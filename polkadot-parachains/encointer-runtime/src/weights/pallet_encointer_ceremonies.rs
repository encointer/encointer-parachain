
//! Autogenerated weights for `pallet_encointer_ceremonies`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// pallet
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_ceremonies
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_ceremonies.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_encointer_ceremonies`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_ceremonies::WeightInfo for WeightInfo<T> {
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:0)
	// Storage: EncointerCeremonies BootstrapperIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableIndex (r:1 w:1)
	// Storage: EncointerCeremonies EndorseeIndex (r:1 w:0)
	// Storage: EncointerCeremonies NewbieIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputationLifetime (r:1 w:0)
	// Storage: EncointerCeremonies ParticipantReputation (r:1 w:2)
	// Storage: EncointerCommunities Bootstrappers (r:1 w:0)
	// Storage: EncointerBalances TotalIssuance (r:1 w:0)
	// Storage: EncointerBalances DemurragePerBlock (r:1 w:0)
	// Storage: EncointerCeremonies ReputableCount (r:1 w:1)
	// Storage: EncointerCeremonies ReputableRegistry (r:0 w:1)
	fn register_participant() -> Weight {
		(183_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCeremonies MeetupCount (r:1 w:0)
	// Storage: EncointerCeremonies AssignmentCounts (r:1 w:0)
	// Storage: EncointerCeremonies Assignments (r:1 w:0)
	// Storage: EncointerCeremonies BootstrapperIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableRegistry (r:8 w:0)
	// Storage: EncointerCeremonies NewbieRegistry (r:2 w:0)
	// Storage: EncointerCommunities Locations (r:2 w:0)
	// Storage: EncointerScheduler PhaseDurations (r:1 w:0)
	// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:0)
	// Storage: EncointerCeremonies MeetupTimeOffset (r:1 w:0)
	// Storage: EncointerCeremonies LocationTolerance (r:1 w:0)
	// Storage: EncointerCeremonies TimeTolerance (r:1 w:0)
	// Storage: EncointerCeremonies AttestationCount (r:1 w:1)
	// Storage: EncointerCeremonies AttestationIndex (r:1 w:1)
	// Storage: EncointerCeremonies MeetupParticipantCountVote (r:0 w:9)
	// Storage: EncointerCeremonies AttestationRegistry (r:0 w:1)
	fn attest_claims() -> Weight {
		(1_221_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:0)
	// Storage: EncointerCeremonies MeetupCount (r:1 w:0)
	// Storage: EncointerCeremonies AssignmentCounts (r:1 w:0)
	// Storage: EncointerCeremonies Assignments (r:1 w:0)
	// Storage: EncointerCeremonies BootstrapperIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableRegistry (r:8 w:0)
	// Storage: EncointerCeremonies NewbieRegistry (r:2 w:0)
	// Storage: EncointerCommunities Locations (r:2 w:0)
	// Storage: EncointerScheduler PhaseDurations (r:1 w:0)
	// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:0)
	// Storage: EncointerCeremonies MeetupTimeOffset (r:1 w:0)
	// Storage: EncointerCeremonies AttestationCount (r:1 w:1)
	// Storage: EncointerCeremonies AttestationIndex (r:1 w:1)
	// Storage: EncointerCeremonies MeetupParticipantCountVote (r:0 w:1)
	// Storage: EncointerCeremonies AttestationRegistry (r:0 w:1)
	fn attest_attendees() -> Weight {
		(224_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerCommunities Bootstrappers (r:1 w:0)
	// Storage: EncointerCeremonies BurnedBootstrapperNewbieTickets (r:1 w:1)
	// Storage: EncointerCeremonies EndorsementTicketsPerBootstrapper (r:1 w:0)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:0)
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCeremonies Endorsees (r:1 w:1)
	// Storage: EncointerCeremonies EndorseesCount (r:1 w:1)
	// Storage: EncointerCeremonies NewbieIndex (r:1 w:1)
	// Storage: EncointerCeremonies NewbieCount (r:1 w:1)
	// Storage: EncointerCeremonies NewbieRegistry (r:1 w:1)
	// Storage: EncointerBalances TotalIssuance (r:1 w:0)
	// Storage: EncointerBalances DemurragePerBlock (r:1 w:0)
	// Storage: EncointerCeremonies EndorseeCount (r:1 w:1)
	// Storage: EncointerCeremonies EndorseeRegistry (r:0 w:1)
	// Storage: EncointerCeremonies EndorseeIndex (r:0 w:1)
	fn endorse_newcomer() -> Weight {
		(158_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:0)
	// Storage: EncointerCeremonies MeetupCount (r:1 w:0)
	// Storage: EncointerCeremonies AssignmentCounts (r:1 w:0)
	// Storage: EncointerCeremonies Assignments (r:1 w:0)
	// Storage: EncointerCeremonies BootstrapperIndex (r:1 w:0)
	// Storage: EncointerCeremonies ReputableIndex (r:1 w:0)
	// Storage: EncointerCeremonies IssuedRewards (r:1 w:1)
	// Storage: EncointerCeremonies ReputableRegistry (r:8 w:0)
	// Storage: EncointerCeremonies NewbieRegistry (r:2 w:0)
	// Storage: EncointerCeremonies AttestationIndex (r:10 w:0)
	// Storage: EncointerCeremonies AttestationRegistry (r:10 w:0)
	// Storage: EncointerCeremonies MeetupParticipantCountVote (r:10 w:0)
	// Storage: EncointerCommunities NominalIncome (r:1 w:0)
	// Storage: EncointerCeremonies CeremonyReward (r:1 w:0)
	// Storage: EncointerBalances Balance (r:10 w:10)
	// Storage: EncointerBalances DemurragePerBlock (r:1 w:0)
	// Storage: EncointerBalances TotalIssuance (r:1 w:1)
	// Storage: EncointerCeremonies ParticipantReputation (r:0 w:10)
	fn claim_rewards() -> Weight {
		(782_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(62 as Weight))
			.saturating_add(T::DbWeight::get().writes(22 as Weight))
	}
	// Storage: EncointerCeremonies InactivityTimeout (r:0 w:1)
	fn set_inactivity_timeout() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerScheduler CurrentPhase (r:1 w:0)
	// Storage: EncointerCeremonies MeetupTimeOffset (r:0 w:1)
	fn set_meetup_time_offset() -> Weight {
		(32_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies ReputationLifetime (r:0 w:1)
	fn set_reputation_lifetime() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies EndorsementTicketsPerBootstrapper (r:0 w:1)
	fn set_endorsement_tickets_per_bootstrapper() -> Weight {
		(30_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies TimeTolerance (r:0 w:1)
	fn set_time_tolerance() -> Weight {
		(32_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies LocationTolerance (r:0 w:1)
	fn set_location_tolerance() -> Weight {
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: EncointerCeremonies Assignments (r:0 w:1)
	// Storage: EncointerCeremonies ParticipantReputation (r:0 w:1)
	// Storage: EncointerCeremonies MeetupCount (r:0 w:1)
	// Storage: EncointerCeremonies ReputableRegistry (r:0 w:1)
	// Storage: EncointerCeremonies NewbieCount (r:0 w:1)
	// Storage: EncointerCeremonies ReputableCount (r:0 w:1)
	// Storage: EncointerCeremonies AssignmentCounts (r:0 w:1)
	// Storage: EncointerCeremonies BootstrapperCount (r:0 w:1)
	// Storage: EncointerCeremonies ReputableIndex (r:0 w:1)
	// Storage: EncointerCeremonies AttestationCount (r:0 w:1)
	// Storage: EncointerCeremonies EndorseeCount (r:0 w:1)
	// Storage: EncointerCeremonies EndorseesCount (r:0 w:1)
	fn purge_community_ceremony() -> Weight {
		(185_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn upgrade_registration() -> Weight {
		(247_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(15 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	
	fn unregister_participant() -> Weight {
		(91_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
    fn set_endorsement_tickets_per_reputable() -> Weight {
        (38_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
