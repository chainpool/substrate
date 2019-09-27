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

//! Substrate system API.

#[cfg(test)]
mod tests;
use std::sync::Arc;
//use futures03::{channel::{mpsc, oneshot}, compat::Compat};
//use api::Receiver;
use sr_primitives::traits::{self, Header as HeaderT};
use self::error::Result;

pub use api::system::*;
pub use self::helpers::{Properties, SystemInfo, Health, PeerInfo};
pub use self::gen_client::Client as SystemClient;

/// System API implementation
pub struct System<B: traits::Block> {
	info: SystemInfo,
	sync: Arc<dyn network::SyncProvider<B>>,
	should_have_peers: bool,
}

impl<B: traits::Block> System<B> {
	/// Creates new `System` given the `SystemInfo`.
	pub fn new(
		info: SystemInfo,
		sync: Arc<dyn network::SyncProvider<B>>,
		should_have_peers: bool,
	) -> Self {
		System {
			info,
			should_have_peers,
			sync,
		}
	}
}

impl<B: traits::Block> SystemApi<B::Hash, <B::Header as HeaderT>::Number> for System<B> {
	fn system_name(&self) -> Result<String> {
		Ok(self.info.impl_name.clone())
	}

	fn system_version(&self) -> Result<String> {
		Ok(self.info.impl_version.clone())
	}

	fn system_chain(&self) -> Result<String> {
		Ok(self.info.chain_name.clone())
	}

	fn system_properties(&self) -> Result<Properties> {
		Ok(self.info.properties.clone())
	}

	fn system_health(&self) -> Result<Health> {
		Ok(Health {
			peers: self.sync.peers().len(),
			is_syncing: self.sync.is_major_syncing(),
			should_have_peers: self.should_have_peers,
		})
	}

	fn system_peers(&self) -> Result<Vec<PeerInfo<B::Hash, <B::Header as HeaderT>::Number>>> {
		Ok(self.sync.peers().into_iter().map(|(peer_id, p)| PeerInfo {
			peer_id: peer_id.to_base58(),
			roles: format!("{:?}", p.roles),
			protocol_version: p.protocol_version,
			best_hash: p.best_hash,
			best_number: p.best_number,
		}).collect())
	}

	fn system_network_state(&self) -> Result<network::NetworkState> {
		Ok(self.sync.network_state())
	}
}
