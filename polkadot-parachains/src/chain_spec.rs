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
use hex_literal::hex;
use parachain_runtime::{AccountId, AuraId, BalanceType, CeremonyPhaseType, Demurrage, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

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

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_chain_spec(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		vec![],
		None,
		None,
		None,
		Extensions { relay_chain: "westend-dev".into(), para_id: id.into() },
	)
}

pub fn encointer_spec(id: ParaId, use_well_known_keys: bool, relay_chain: RelayChain) -> ChainSpec {
	// encointer_root
	let mut root_account: AccountId =
		hex!["107f9c5385955bc57ac108b46b36498c4a8348eb964258b9b2ac53797d94794b"].into();
	let mut endowed_accounts = vec![root_account.clone()];

	if use_well_known_keys {
		root_account = get_account_id_from_seed::<sr25519::Public>("Alice");
		endowed_accounts = vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
		];
	}

	ChainSpec::from_genesis(
		"Encointer PC1",
		"encointer-rococo-v1",
		relay_chain.chain_type(),
		move || {
			testnet_genesis(
				root_account.clone(),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				endowed_accounts.clone(),
				id,
			)
		},
		Vec::new(),
		// telemetry endpoints
		None,
		// protocol id
		Some("encointer-rococo-v1"),
		// properties
		Some(
			serde_json::from_str(
				r#"{
			"ss58Format": 42,
			"tokenDecimals": 12,
			"tokenSymbol": "ERT"
		  }"#,
			)
			.unwrap(),
		),
		Extensions { relay_chain: relay_chain.to_string(), para_id: id.into() },
	)
}

pub fn sybil_dummy_spec(id: ParaId, relay_chain: RelayChain) -> ChainSpec {
	let root_account = get_account_id_from_seed::<sr25519::Public>("Alice");
	let endowed_accounts = vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
	];

	ChainSpec::from_genesis(
		"Sybil Dummy",
		"sybil-dummy-rococo-v1",
		relay_chain.chain_type(),
		move || {
			testnet_genesis(
				root_account.clone(),
				vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
				endowed_accounts.clone(),
				id,
			)
		},
		Vec::new(),
		// telemetry endpoints
		None,
		// protocol id
		Some("sybil-dummy-rococo-v1"),
		// properties
		Some(
			serde_json::from_str(
				r#"{
			"ss58Format": 42,
			"tokenDecimals": 12,
			"tokenSymbol": "DUM"
		  }"#,
			)
			.unwrap(),
		),
		Extensions { relay_chain: relay_chain.to_string(), para_id: id.into() },
	)
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	parachain_runtime::GenesisConfig {
		system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		sudo: parachain_runtime::SudoConfig { key: root_key.clone() },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
		aura: parachain_runtime::AuraConfig { authorities: initial_authorities },
		aura_ext: Default::default(),
		encointer_scheduler: parachain_runtime::EncointerSchedulerConfig {
			current_phase: CeremonyPhaseType::REGISTERING,
			current_ceremony_index: 1,
			ceremony_master: root_key.clone(),
			phase_durations: vec![
				(CeremonyPhaseType::REGISTERING, 600_000),
				(CeremonyPhaseType::ASSIGNING, 600_000),
				(CeremonyPhaseType::ATTESTING, 600_000),
			],
		},
		encointer_ceremonies: parachain_runtime::EncointerCeremoniesConfig {
			ceremony_reward: BalanceType::from_num(1),
			time_tolerance: 600_000,   // +-10min
			location_tolerance: 1_000, // [m]
		},
		encointer_communities: parachain_runtime::EncointerCommunitiesConfig {
			community_master: root_key,
		},
		encointer_balances: parachain_runtime::EncointerBalancesConfig {
			demurrage_per_block_default: Demurrage::from_bits(
				0x0000000000000000000001E3F0A8A973_i128,
			),
		},
	}
}

pub enum RelayChain {
	RococoLocal,
	// Kusama,
	// KusamaLocal,
	// PolkadotLocal,
	Rococo,
	// Polkadot,
}

impl ToString for RelayChain {
	fn to_string(&self) -> String {
		match self {
			RelayChain::RococoLocal => "rococo-local".into(),
			// RelayChain::Kusama => "kusama".into(),
			// RelayChain::KusamaLocal => "kusama-local".into(),
			// RelayChain::PolkadotLocal => "polkadot-local".into(),
			RelayChain::Rococo => "rococo".into(),
			// RelayChain::Polkadot => "polkadot".into(),
		}
	}
}

impl RelayChain {
	fn chain_type(&self) -> ChainType {
		match self {
			RelayChain::RococoLocal => ChainType::Local,
			// RelayChain::KusamaLocal => ChainType::Local,
			// RelayChain::PolkadotLocal => ChainType::Local,
			// RelayChain::Kusama => ChainType::Live,
			RelayChain::Rococo => ChainType::Live,
			// RelayChain::Polkadot => ChainType::Live,
		}
	}
}
