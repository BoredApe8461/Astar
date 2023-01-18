// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_dapps_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `shiden-collator-02-ovh`, CPU: `Intel(R) Xeon(R) E-2136 CPU @ 3.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("astar-dev"), DB CACHE: 1024

// Executed Command:
// ./astar-collator
// benchmark
// pallet
// --chain
// astar-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --pallet
// pallet_dapps_staking
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallet_dapps_staking_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_dapps_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dapps_staking::WeightInfo for WeightInfo<T> {
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	fn register() -> Weight {
		Weight::from_ref_time(34_762_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unregister() -> Weight {
		Weight::from_ref_time(37_244_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_from_unregistered() -> Weight {
		Weight::from_ref_time(53_956_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		Weight::from_ref_time(140_275_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn unbond_and_unstake() -> Weight {
		Weight::from_ref_time(141_641_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:1)
	fn withdraw_unbonded() -> Weight {
		Weight::from_ref_time(116_564_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:2 w:0)
	// Storage: DappsStaking GeneralStakerInfo (r:2 w:2)
	// Storage: DappsStaking ContractEraStake (r:2 w:2)
	fn nomination_transfer() -> Weight {
		Weight::from_ref_time(43_519_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:2 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:2 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_staker_with_restake() -> Weight {
		Weight::from_ref_time(72_500_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: DappsStaking GeneralStakerInfo (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:0)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:0)
	fn claim_staker_without_restake() -> Weight {
		Weight::from_ref_time(45_414_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking GeneralEraInfo (r:1 w:0)
	fn claim_dapp() -> Weight {
		Weight::from_ref_time(35_939_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn force_new_era() -> Weight {
		Weight::from_ref_time(6_910_000 as u64)
	}
	fn maintenance_mode() -> Weight {
		Weight::from_ref_time(14_658_000 as u64)
	}
	// Storage: DappsStaking Ledger (r:1 w:1)
	fn set_reward_destination() -> Weight {
		Weight::from_ref_time(23_412_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
