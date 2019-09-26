// Copyright 2017-2019 Parity Technologies (UK) Ltd.
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

//! Contract execution data.

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "std")]
use crate::bytes;
use rstd::vec::Vec;

/// Contract storage key.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug, Hash, PartialOrd, Ord, Clone))]
pub struct StorageKey(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);

/// Contract storage entry data.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug, Hash, PartialOrd, Ord, Clone))]
pub struct StorageData(#[cfg_attr(feature = "std", serde(with="bytes"))] pub Vec<u8>);

/// Storage change set
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug, PartialEq, Eq))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct StorageChangeSet<Hash> {
	/// Block hash
	pub block: Hash,
	/// A list of changes
	pub changes: Vec<(
		StorageKey,
		Option<StorageData>,
	)>,
}

/// List of all well known keys and prefixes in storage.
pub mod well_known_keys {

	/// Wasm code of the runtime.
	///
	/// Stored as a raw byte vector. Required by substrate.
	pub const CODE: &'static [u8] = b":code";

	/// Number of wasm linear memory pages required for execution of the runtime.
	///
	/// The type of this value is encoded `u64`.
	pub const HEAP_PAGES: &'static [u8] = b":heappages";

	/// Number of authorities.
	///
	/// The type of this value is encoded `u32`. Required by substrate.
	pub const AUTHORITY_COUNT: &'static [u8] = b":auth:len";

	/// Prefix under which authorities are storied.
	///
	/// The full key for N-th authority is generated as:
	///
	/// `(n as u32).to_keyed_vec(AUTHORITY_PREFIX)`.
	pub const AUTHORITY_PREFIX: &'static [u8] = b":auth:";

	/// Current extrinsic index (u32) is stored under this key.
	pub const EXTRINSIC_INDEX: &'static [u8] = b":extrinsic_index";

	/// Changes trie configuration is stored under this key.
	pub const CHANGES_TRIE_CONFIG: &'static [u8] = b":changes_trie";

	/// Prefix of child storage keys.
	pub const CHILD_STORAGE_KEY_PREFIX: &'static [u8] = b":child_storage:";

	/// Whether a key is a child storage key.
	///
	/// This is convenience function which basically checks if the given `key` starts
	/// with `CHILD_STORAGE_KEY_PREFIX` and doesn't do anything apart from that.
	pub fn is_child_storage_key(key: &[u8]) -> bool {
		// Other code might depend on this, so be careful changing this.
		key.starts_with(CHILD_STORAGE_KEY_PREFIX)
	}

	/// Determine whether a child trie key is valid.
	///
	/// For now, the only valid child trie keys are those starting with `:child_storage:default:`.
	///
	/// `child_trie_root` and `child_delta_trie_root` can panic if invalid value is provided to them.
	pub fn is_child_trie_key_valid(storage_key: &[u8]) -> bool {
		let has_right_prefix = storage_key.starts_with(b":child_storage:default:");
		if has_right_prefix {
			// This is an attempt to catch a change of `is_child_storage_key`, which
			// just checks if the key has prefix `:child_storage:` at the moment of writing.
			debug_assert!(
				is_child_storage_key(&storage_key),
				"`is_child_trie_key_valid` is a subset of `is_child_storage_key`",
			);
		}
		has_right_prefix
	}
}
