// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use cumulus_primitives_core::ParaId;
use parachain_runtime::{BalanceType, CeremonyPhaseType};
use parachains_common::{AccountId, AuraId, Balance};
use parity_scale_codec::{Decode, Encode};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};

pub use crate::chain_spec_helpers::{EncointerKeys, GenesisKeys, RelayChain, WellKnownKeys};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

pub const ENDOWED_FUNDING: u128 = 1 << 60;
pub const ENCOINTER_KUSAMA_ED: Balance = parachain_runtime::ExistentialDeposit::get();

/// Configure `endowed_accounts` with initial balance of `ENDOWED_FUNDING`.
pub fn allocate_endowance(endowed_accounts: Vec<AccountId>) -> Vec<(AccountId, u128)> {
	endowed_accounts.into_iter().map(|k| (k, ENDOWED_FUNDING)).collect()
}

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// Chain-spec for the encointer runtime
pub fn encointer_spec(
	para_id: ParaId,
	genesis_keys: GenesisKeys,
	relay_chain: RelayChain,
) -> ChainSpec {
	let (council, endowed, authorities) = match genesis_keys {
		GenesisKeys::Encointer =>
			(EncointerKeys::council(), [].to_vec(), EncointerKeys::invulnerables()),
		GenesisKeys::EncointerWithCouncilEndowed =>
			(EncointerKeys::council(), EncointerKeys::council(), EncointerKeys::invulnerables()),
		GenesisKeys::WellKnown =>
			(WellKnownKeys::council(), WellKnownKeys::endowed(), WellKnownKeys::invulnerables()),
	};

	#[allow(deprecated)]
	ChainSpec::builder(
		parachain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: relay_chain.to_string(), para_id: para_id.into() },
	)
	.with_name("Encointer Network")
	.with_id(&format!("encointer-{}", relay_chain.to_string()))
	.with_protocol_id(&format!("nctr-{}", relay_chain.to_string().chars().next().unwrap()))
	.with_chain_type(relay_chain.chain_type())
	.with_properties(relay_chain.properties())
	.with_genesis_config_patch(encointer_genesis(
		council.clone(),
		authorities.clone(),
		allocate_endowance(endowed.clone()),
		para_id,
	))
	.build()
}

pub fn sybil_dummy_spec(para_id: ParaId, relay_chain: RelayChain) -> ChainSpec {
	let (council, endowed, authorities) =
		(WellKnownKeys::council(), WellKnownKeys::endowed(), WellKnownKeys::invulnerables());
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "DUM".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 13.into());

	#[allow(deprecated)]
	ChainSpec::builder(
		parachain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: relay_chain.to_string(), para_id: para_id.into() },
	)
	.with_name("Sybil Dummy")
	.with_id(&format!("sybil-dummy-{}", relay_chain.to_string()))
	.with_protocol_id(&format!("nctr-{}", relay_chain.to_string().chars().next().unwrap()))
	.with_chain_type(relay_chain.chain_type())
	.with_properties(properties)
	.with_genesis_config_patch(encointer_genesis(
		council.clone(),
		authorities.clone(),
		allocate_endowance(endowed.clone()),
		para_id,
	))
	.build()
}

fn encointer_genesis(
	encointer_council: Vec<AccountId>,
	invulnerables: Vec<AccountId>,
	endowance_allocation: Vec<(AccountId, u128)>,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowance_allocation,
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"collatorSelection": parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.clone(),
			candidacy_bond: ENCOINTER_KUSAMA_ED * 16,
			..Default::default()
		},
		"session": parachain_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|acc| {
					(
						acc.clone(),                         // account id
						acc.clone(),                                 // validator id
						parachain_runtime::SessionKeys { aura: Decode::decode(&mut acc.encode().as_ref()).unwrap() }, // session keys
					)
				})
				.collect(),
		},
		"membership": {
			"members": encointer_council,
		},
		"encointerScheduler": {
			"currentPhase": CeremonyPhaseType::Registering,
			"currentCeremonyIndex": 1,
			"phaseDurations": vec![
				(CeremonyPhaseType::Registering, 604800000u64), // 7d
				(CeremonyPhaseType::Assigning, 86400000u64),    // 1d
				(CeremonyPhaseType::Attesting, 172800000u64),   // 2d
			],
		},
		"encointerCeremonies": {
			"ceremonyReward": BalanceType::from_num(1),
			"timeTolerance": 600_000u64,   // +-10min
			"locationTolerance": 1_000, // [m]
			"endorsementTicketsPerBootstrapper": 10,
			"endorsementTicketsPerReputable": 5,
			"reputationLifetime": 5,
			"inactivityTimeout": 5, // idle ceremonies before purging community
			"meetupTimeOffset": 0,
		},
		"encointerCommunities": {
			"minSolarTripTimeS": 1, // [s]
			"maxSpeedMps": 1,         // [m/s] suggested would be 83m/s for security,
		},
		"encointerBalances": {
			// for relative adjustment.
			"feeConversionFactor": 7_143u32,
		},
		"encointerFaucet": {
			"reserveAmount": 10_000_000_000_000u128,
		},
	})
}

/// hard-coded runtime config for rococo
pub fn encointer_rococo() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/encointer-rococo.json")[..])
}

/// hard-coded runtime config for kusama
pub fn encointer_kusama() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/encointer-kusama.json")[..])
}

/// hard-coded runtime config for westend
pub fn encointer_westend() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/encointer-westend.json")[..])
}
