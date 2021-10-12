// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for kilt_launch
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-12, STEPS: {{cmd.steps}}\, REPEAT: {{cmd.repeat}}\, LOW RANGE: {{cmd.lowest_range_values}}\, HIGH RANGE: {{cmd.highest_range_values}}\
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/kilt-parachain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=kilt_launch
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=pallets/kilt-launch/src/default_weights.rs
// --template=.maintain/weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for kilt_launch.
pub trait WeightInfo {
	fn change_transfer_account() -> Weight;
	fn force_unlock(n: u32, ) -> Weight;
	fn locked_transfer() -> Weight;
	fn migrate_genesis_account_vesting() -> Weight;
	fn migrate_genesis_account_locking() -> Weight;
	fn migrate_multiple_genesis_accounts_vesting(n: u32, ) -> Weight;
	fn migrate_multiple_genesis_accounts_locking(n: u32, ) -> Weight;
}

/// Weights for kilt_launch using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn change_transfer_account() -> Weight {
		(3_310_000_u64)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn force_unlock(n: u32, ) -> Weight {
		(26_073_000_u64)
			// Standard Error: 16_000
			.saturating_add((28_680_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn locked_transfer() -> Weight {
		(138_525_000_u64)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_vesting() -> Weight {
		(150_627_000_u64)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_locking() -> Weight {
		(158_147_000_u64)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	fn migrate_multiple_genesis_accounts_vesting(n: u32, ) -> Weight {
		(48_004_000_u64)
			// Standard Error: 45_000
			.saturating_add((96_817_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn migrate_multiple_genesis_accounts_locking(n: u32, ) -> Weight {
		(58_019_000_u64)
			// Standard Error: 44_000
			.saturating_add((98_764_000_u64).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn change_transfer_account() -> Weight {
		(3_310_000_u64)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn force_unlock(n: u32, ) -> Weight {
		(26_073_000_u64)
			// Standard Error: 16_000
			.saturating_add((28_680_000_u64).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn locked_transfer() -> Weight {
		(138_525_000_u64)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_vesting() -> Weight {
		(150_627_000_u64)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	fn migrate_genesis_account_locking() -> Weight {
		(158_147_000_u64)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	fn migrate_multiple_genesis_accounts_vesting(n: u32, ) -> Weight {
		(48_004_000_u64)
			// Standard Error: 45_000
			.saturating_add((96_817_000_u64).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
	fn migrate_multiple_genesis_accounts_locking(n: u32, ) -> Weight {
		(58_019_000_u64)
			// Standard Error: 44_000
			.saturating_add((98_764_000_u64).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((5_u64).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n as Weight)))
	}
}
