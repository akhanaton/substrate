// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Test utilities

#![cfg(test)]

use primitives::{traits::IdentityLookup, BuildStorage, Perbill};
use primitives::testing::{Digest, DigestItem, Header, UintAuthorityId, ConvertUintAuthorityId};
use substrate_primitives::{H256, Blake2Hasher};
use runtime_io;
use srml_support::impl_outer_origin;
use crate::{GenesisConfig, Module, Trait};

impl_outer_origin!{
	pub enum Origin for Test {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
impl consensus::Trait for Test {
	type Log = DigestItem;
	type SessionKey = UintAuthorityId;
	type InherentOfflineReport = ();
}
impl system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::primitives::traits::BlakeTwo256;
	type Digest = Digest;
	type AccountId = u64;
	type Lookup = IdentityLookup<u64>;
	type Header = Header;
	type Event = ();
	type Log = DigestItem;
}
impl balances::Trait for Test {
	type Balance = u64;
	type OnFreeBalanceZero = Staking;
	type OnNewAccount = ();
	type EnsureAccountLiquid = Staking;
	type Event = ();
}
impl session::Trait for Test {
	type ConvertAccountIdToSessionKey = ConvertUintAuthorityId;
	type OnSessionChange = Staking;
	type Event = ();
}
impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
}
impl Trait for Test {
	type Currency = balances::Module<Self>;
	type OnRewardMinted = ();
	type Event = ();
}

pub fn new_test_ext(
	ext_deposit: u64,
	session_length: u64,
	sessions_per_era: u64,
	current_era: u64,
	monied: bool,
	reward: u64
) -> runtime_io::TestExternalities<Blake2Hasher> {
	let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
	let balance_factor = if ext_deposit > 0 {
		256
	} else {
		1
	};
	t.extend(consensus::GenesisConfig::<Test>{
		code: vec![],
		authorities: vec![],
	}.build_storage().unwrap().0);
	t.extend(session::GenesisConfig::<Test>{
		session_length,
		validators: vec![10, 20],
	}.build_storage().unwrap().0);
	t.extend(balances::GenesisConfig::<Test>{
		balances: if monied {
			if reward > 0 {
				vec![(1, 10 * balance_factor), (2, 20 * balance_factor), (3, 30 * balance_factor), (4, 40 * balance_factor), (10, balance_factor), (20, balance_factor)]
			} else {
				vec![(1, 10 * balance_factor), (2, 20 * balance_factor), (3, 30 * balance_factor), (4, 40 * balance_factor)]
			}
		} else {
			vec![(10, balance_factor), (20, balance_factor)]
		},
		existential_deposit: ext_deposit,
		transfer_fee: 0,
		creation_fee: 0,
		vesting: vec![],
	}.build_storage().unwrap().0);
	t.extend(GenesisConfig::<Test>{
		sessions_per_era,
		current_era,
		intentions: vec![10, 20],
		validator_count: 2,
		minimum_validator_count: 0,
		bonding_duration: sessions_per_era * session_length * 3,
		session_reward: Perbill::from_millionths((1000000 * reward / balance_factor) as u32),
		offline_slash: if monied { Perbill::from_percent(40) } else { Perbill::zero() },
		current_session_reward: reward,
		current_offline_slash: 20,
		offline_slash_grace: 0,
		invulnerables: vec![],
	}.build_storage().unwrap().0);
	t.extend(timestamp::GenesisConfig::<Test>{
		period: 5,
	}.build_storage().unwrap().0);
	runtime_io::TestExternalities::new(t)
}

pub type System = system::Module<Test>;
pub type Balances = balances::Module<Test>;
pub type Session = session::Module<Test>;
pub type Timestamp = timestamp::Module<Test>;
pub type Staking = Module<Test>;
