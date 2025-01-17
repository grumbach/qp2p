// Copyright 2021 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::{Config, ConnId, QuicP2p};
use anyhow::Result;
use bytes::Bytes;
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tiny_keccak::{Hasher, Sha3};

/// SHA3-256 hash digest.
type Digest256 = [u8; 32];

mod common;
mod quinn;

impl ConnId for [u8; 32] {
    fn generate(_socket_addr: &SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(rand::random())
    }
}

/// Constructs a `QuicP2p` node with some sane defaults for testing.
pub(crate) fn new_qp2p() -> Result<QuicP2p<[u8; 32]>> {
    new_qp2p_with_hcc(HashSet::default())
}

pub(crate) fn new_qp2p_with_hcc(
    hard_coded_contacts: HashSet<SocketAddr>,
) -> Result<QuicP2p<[u8; 32]>> {
    let qp2p = QuicP2p::<[u8; 32]>::with_config(
        Some(Config {
            local_port: Some(0),
            local_ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
            hard_coded_contacts,
            // turn down the retry duration - we won't live forever
            // note that this would just limit retries, UDP connection attempts seem to take 60s to
            // timeout
            retry_duration_msec: 500,
            ..Config::default()
        }),
        // Make sure we start with an empty cache. Otherwise, we might get into unexpected state.
        Default::default(),
        true,
    )?;

    Ok(qp2p)
}

pub(crate) fn random_msg(size: usize) -> Bytes {
    let random_bytes: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();
    Bytes::from(random_bytes)
}

pub(crate) fn hash(bytes: &Bytes) -> Digest256 {
    let mut hasher = Sha3::v256();
    let mut hash = Digest256::default();
    hasher.update(bytes);
    hasher.finalize(&mut hash);
    hash
}
