//! Currency model used in encointer's runtimes.
//!
//! Copied from statemine/src/constants but removed some parts.

use node_primitives::Balance;

/// The existential deposit. Set to 1/10 of its parent Relay Chain (v9020).
pub const EXISTENTIAL_DEPOSIT: Balance = CENTS / 10;

pub const UNITS: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNITS / 30_000;
pub const GRAND: Balance = CENTS * 100_000;
pub const MILLICENTS: Balance = CENTS / 1_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	// map to 1/10 of what the kusama relay chain charges (v9020)
	(items as Balance * 2_000 * CENTS + (bytes as Balance) * 100 * MILLICENTS) / 10
}
