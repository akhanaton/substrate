// Copyright 2017-2018 Parity Technologies (UK) Ltd.
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

//! An implementation of double map backed by storage.

use crate::rstd::prelude::*;
use crate::codec::{Codec, Encode};
use crate::storage::unhashed;

/// An implementation of a map with a two keys.
///
/// It provides an important ability to efficiently remove all entries
/// that have a common first key.
///
/// # Mapping of keys to a storage path
///
/// The storage key (i.e. the key under which the `Value` will be stored) is created from two parts.
/// The first part is a hash of a concatenation of the `PREFIX` and `Key1`. And the second part
/// is a hash of a `Key2`.
pub trait StorageDoubleMap {
	type Key1: Codec;
	type Key2: Codec;
	type Value: Codec + Default;

	const PREFIX: &'static [u8];

	/// Insert an entry into this map.
	fn insert(k1: Self::Key1, k2: Self::Key2, val: Self::Value) {
		unhashed::put(&Self::full_key(k1, k2)[..], &val);
	}

	/// Remove an entry from this map.
	fn remove(k1: Self::Key1, k2: Self::Key2) {
		unhashed::kill(&Self::full_key(k1, k2)[..]);
	}

	/// Get an entry from this map.
	///
	/// If there is entry stored under the given keys, returns `None`.
	fn get(k1: Self::Key1, k2: Self::Key2) -> Option<Self::Value> {
		unhashed::get(&Self::full_key(k1, k2)[..])
	}

	/// Returns `true` if value under the specified keys exists.
	fn exists(k1: Self::Key1, k2: Self::Key2) -> bool {
		unhashed::exists(&Self::full_key(k1, k2)[..])
	}

	/// Removes all entries that shares the `k1` as the first key.
	fn remove_prefix(k1: Self::Key1) {
		unhashed::kill_prefix(&Self::derive_key1(Self::encode_key1(k1)))
	}

	/// Encode key1 into Vec<u8> and prepend a prefix
	fn encode_key1(key: Self::Key1) -> Vec<u8> {
		let mut raw_prefix = Vec::new();
		raw_prefix.extend(Self::PREFIX);
		raw_prefix.extend(Encode::encode(&key));
		raw_prefix
	}

	/// Encode key2 into Vec<u8>
	fn encode_key2(key: Self::Key2) -> Vec<u8> {
		Encode::encode(&key)
	}

	/// Derive the first part of the key
	fn derive_key1(key1_data: Vec<u8>) -> Vec<u8>;

	/// Derive the remaining part of the key
	fn derive_key2(key2_data: Vec<u8>) -> Vec<u8>;

	/// Returns a compound key that consist of the two parts: (prefix, `k1`) and `k2`.
	/// The first part is hased and then concatenated with a hash of `k2`.
	fn full_key(k1: Self::Key1, k2: Self::Key2) -> Vec<u8> {
		let key1_data = Self::encode_key1(k1);
		let key2_data = Self::encode_key2(k2);
		let mut key = Self::derive_key1(key1_data);
		key.extend(Self::derive_key2(key2_data));
		key
	}
}
