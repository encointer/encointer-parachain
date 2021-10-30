// Copyright (c) 2019 Alain Brenzikofer
// This file is part of Encointer
//
// Encointer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Encointer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Encointer.  If not, see <http://www.gnu.org/licenses/>.

//! Some helpers to create chains-specs

use cumulus_primitives_core::ParaId;
use parachain_runtime::{AccountId, AuraId};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, GenericChainSpec};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::Ss58Codec, sr25519, Public};
use sp_keyring::AccountKeyring::{Alice, Bob, Dave, Eve};
use std::str::FromStr;

pub fn public_from_ss58<TPublic: Public + FromStr>(ss58: &str) -> TPublic
	where
		<TPublic as FromStr>::Err: std::fmt::Debug,
{
	TPublic::from_ss58check(ss58).expect("supply valid ss58!")
}


/// Defines the key set to use for root, endowed accounts, or authorities.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum GenesisKeys {
	/// Use Encointer keys.
	Encointer,
	/// Use Keys from the keyring for a test setup
	WellKnown,
}

struct WellKnownKeys;

impl WellKnownKeys {
	fn root() -> AccountId {
		Alice.to_account_id()
	}

	fn endowed() -> Vec<AccountId> {
		vec![Alice.to_account_id(), Bob.to_account_id()]
	}

	fn authorities() -> Vec<AuraId> {
		vec![Alice.public().into(), Eve.public().into()]
	}
}

struct EncointerKeys;

impl EncointerKeys {
	fn root() -> AccountId {
		public_from_ss58::<sr25519::Public>("5EqGFRTN3m2kLpoaThANra5REs5C7B2rfLmmZv2nbJsxaTe1")
			.into()
	}
	fn authorities() -> Vec<AuraId> {
		vec![
			public_from_ss58::<sr25519::Public>("5GZJjbPPD9u6NDgK1ApYmbyGs7EBX4HeEz2y2CD38YJxjvQH")
				.into(),
			/*
			public_from_ss58::<sr25519::Public>("5CcSd1GZus6Jw7rP47LLqMMmtr2KeXCH6W11ZKk1LbCQ9dPY").into(),
			public_from_ss58::<sr25519::Public>("5FsECrDjBXrh5hXmN4PhQfNPbjYYwwW7edu2UQ8G5LR1JFuH").into(),
			public_from_ss58::<sr25519::Public>("5HBdSEnswkqm6eoHzzX5PCeKoC15CCy88vARrT8XMaRRuyaE").into(),
			public_from_ss58::<sr25519::Public>("5GGxVLYTXS7JZAwVzisdXbsugHSD6gtDb3AT3MVzih9jTLQT").into(),

			 */
		]
	}
}