// Copyright 2019 Parity Technologies (UK) Ltd.
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
use parachain_runtime::{
	BalanceType, CeremonyPhaseType, EncointerCeremoniesConfig, EncointerCommunitiesConfig,
	EncointerSchedulerConfig,
};
use rococo_parachain_primitives::{AccountId, Balance, Signature};
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

/// Helper function to reduce boilerplate
fn endow(accounts: &Vec<AccountId>, balance: Balance) -> Vec<(AccountId, Balance)> {
	accounts.iter().cloned().map(|k| (k, balance)).collect()
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
				endow(
					&vec![
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
					1 << 60,
				),
				id,
			)
		},
		vec![],
		None,
		None,
		None,
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn staging_test_net(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
				vec![(
					hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
					(1 << 60),
				)],
				id,
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Extensions {
			relay_chain: "westend-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn encointer_spec(id: ParaId, use_well_known_keys: bool) -> ChainSpec {
	// encointer_root
	let mut root_account: AccountId =
		hex!["107f9c5385955bc57ac108b46b36498c4a8348eb964258b9b2ac53797d94794b"].into();
	let mut endowed_accounts = vec![root_account.clone()];
	let mut chain_type = ChainType::Live;

	if use_well_known_keys {
		root_account = get_account_id_from_seed::<sr25519::Public>("Alice");
		endowed_accounts = vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
		];
		chain_type = ChainType::Local;
	}

	ChainSpec::from_genesis(
		"Encointer PC1",
		"encointer-rococo-v1",
		chain_type,
		move || testnet_genesis(root_account.clone(), endow(&endowed_accounts, 1 << 60), id),
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
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

pub fn sybil_dummy_spec(id: ParaId) -> ChainSpec {
	let root_account = get_account_id_from_seed::<sr25519::Public>("Alice");
	let endowed_accounts = vec![
		(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			(1 << 60),
		),
		(
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			10u128.pow(12),
		),
	];

	ChainSpec::from_genesis(
		"Sybil Dummy",
		"sybil-dummy-rococo-v1",
		ChainType::Local,
		move || testnet_genesis(root_account.clone(), endowed_accounts.clone(), id),
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
		Extensions {
			relay_chain: "rococo".into(),
			para_id: id.into(),
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
	parachain_runtime::GenesisConfig {
		frame_system: Some(parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(parachain_runtime::BalancesConfig {
			balances: endowed_accounts,
		}),
		pallet_sudo: Some(parachain_runtime::SudoConfig {
			key: root_key.clone(),
		}),
		parachain_info: Some(parachain_runtime::ParachainInfoConfig { parachain_id: id }),
		encointer_scheduler: Some(EncointerSchedulerConfig {
			current_phase: CeremonyPhaseType::REGISTERING,
			current_ceremony_index: 1,
			ceremony_master: root_key.clone(),
			phase_durations: vec![
				(CeremonyPhaseType::REGISTERING, 600_000),
				(CeremonyPhaseType::ASSIGNING, 600_000),
				(CeremonyPhaseType::ATTESTING, 600_000),
			],
		}),
		encointer_ceremonies: Some(EncointerCeremoniesConfig {
			ceremony_reward: BalanceType::from_num(1),
			time_tolerance: 600_000,   // +-10min
			location_tolerance: 1_000, // [m]
		}),
		encointer_communities: Some(EncointerCommunitiesConfig {
			community_master: root_key,
		}),
	}
}
