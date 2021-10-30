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
use hex_literal::hex;
use parachain_runtime::{AccountId, AuraId};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::{ChainType, GenericChainSpec};
use serde::{Deserialize, Serialize};
use sp_core::{crypto::Ss58Codec, sr25519, Public};
use sp_keyring::AccountKeyring::{Alice, Bob};
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
		vec![Alice.public().into()]
	}
}

struct EncointerKeys;

impl EncointerKeys {
	fn root() -> AccountId {
		hex!["107f9c5385955bc57ac108b46b36498c4a8348eb964258b9b2ac53797d94794b"].into()
	}
	fn authorities() -> Vec<AuraId> {
		vec![
			public_from_ss58::<sr25519::Public>("5ECixNNkkfjHYqzwEkbuoVdzRqBpW2eTp8rp2SYR8fuNfQ4G")
				.into(),
			public_from_ss58::<sr25519::Public>("5CMekcxVqQ1ziRHoibG2w5Co7wXu7LWXtX7yTK67NWrJ61a9")
				.into(),
			public_from_ss58::<sr25519::Public>("5Gdh3vLvFKPMwMf2h4sngMgxSnaYGZUJTPGkGwoVmZFM2Ss5")
				.into(),
			public_from_ss58::<sr25519::Public>("5DhVfSunCNHy1R1ozJx1V59YbjDAEzEsaAghmzE77opGVUNf")
				.into(),
			public_from_ss58::<sr25519::Public>("5EWpnnj53PL9KbJAMnsrezQYZhwQ6UwnqSknnXd1ptVvRfVJ")
				.into(),
		]
	}
}
