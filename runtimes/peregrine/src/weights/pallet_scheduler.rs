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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-17, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain
// dev
// --heap-pages
// 4096
// --extrinsic
// *
// --pallet
// pallet_scheduler
// --steps
// 50
// --repeat
// 20
// --execution
// wasm
// --wasm-execution
// Compiled
// --output
// runtimes/parachain/src/weights/pallet_scheduler.rs
// --template
// .maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_scheduler using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	fn schedule(s: u32, ) -> Weight {
		37_341_000_u64
			// Standard Error: 3_000
			.saturating_add(148_000_u64.saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn cancel(s: u32, ) -> Weight {
		37_805_000_u64
			// Standard Error: 8_000
			.saturating_add(1_445_000_u64.saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn schedule_named(s: u32, ) -> Weight {
		48_341_000_u64
			// Standard Error: 2_000
			.saturating_add(130_000_u64.saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn cancel_named(s: u32, ) -> Weight {
		40_560_000_u64
			// Standard Error: 8_000
			.saturating_add(1_498_000_u64.saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}