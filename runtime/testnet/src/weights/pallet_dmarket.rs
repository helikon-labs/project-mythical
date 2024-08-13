
//! Autogenerated weights for `pallet_dmarket`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bdl-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local-v")`, DB CACHE: 1024

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --chain
// local-v
// --pallet
// pallet_dmarket
// --extrinsic
// *
// --wasm-execution
// compiled
// --steps
// 50
// --repeat
// 20
// --output
// ./runtime/testnet/src/weights/pallet_dmarket.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_dmarket`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dmarket::WeightInfo for WeightInfo<T> {
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(169), added: 2644, mode: `MaxEncodedLen`)
	/// Storage: `Dmarket::DmarketCollection` (r:1 w:1)
	/// Proof: `Dmarket::DmarketCollection` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	fn force_set_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `444`
		//  Estimated: `3634`
		// Minimum execution time: 22_571_000 picoseconds.
		Weight::from_parts(23_310_000, 0)
			.saturating_add(Weight::from_parts(0, 3634))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Dmarket::DmarketCollection` (r:1 w:0)
	/// Proof: `Dmarket::DmarketCollection` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Item` (r:1 w:1)
	/// Proof: `Nfts::Item` (`max_values`: None, `max_size`: Some(637), added: 3112, mode: `MaxEncodedLen`)
	/// Storage: `Dmarket::ClosedAsks` (r:1 w:1)
	/// Proof: `Dmarket::ClosedAsks` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Dmarket::ClosedBids` (r:1 w:1)
	/// Proof: `Dmarket::ClosedBids` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Collection` (r:1 w:0)
	/// Proof: `Nfts::Collection` (`max_values`: None, `max_size`: Some(169), added: 2644, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Attribute` (r:1 w:0)
	/// Proof: `Nfts::Attribute` (`max_values`: None, `max_size`: Some(495), added: 2970, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::CollectionConfigOf` (r:1 w:0)
	/// Proof: `Nfts::CollectionConfigOf` (`max_values`: None, `max_size`: Some(142), added: 2617, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemConfigOf` (r:1 w:0)
	/// Proof: `Nfts::ItemConfigOf` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::Account` (r:0 w:2)
	/// Proof: `Nfts::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::ItemPriceOf` (r:0 w:1)
	/// Proof: `Nfts::ItemPriceOf` (`max_values`: None, `max_size`: Some(117), added: 2592, mode: `MaxEncodedLen`)
	/// Storage: `Nfts::PendingSwapOf` (r:0 w:1)
	/// Proof: `Nfts::PendingSwapOf` (`max_values`: None, `max_size`: Some(151), added: 2626, mode: `MaxEncodedLen`)
	fn execute_trade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1205`
		//  Estimated: `4102`
		// Minimum execution time: 321_303_000 picoseconds.
		Weight::from_parts(325_174_000, 0)
			.saturating_add(Weight::from_parts(0, 4102))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
