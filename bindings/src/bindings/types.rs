//! Types passed across the Javascript/Webassembly boundary.
use serde::{Deserialize, Serialize};

use mpc_driver::gg20;
use mpc_protocol::{Keypair, Parameters, SessionId};

/// Supported multi-party computation protocols.
#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Protocol {
    /// The GG2020 protocol.
    #[serde(rename = "gg20")]
    GG20,
    /// The CGGMP protocol.
    #[serde(rename = "cggmp")]
    CGGMP,
}

/// Generated key share.
#[derive(Serialize, Deserialize)]
pub struct KeyShare {
    /// Private key share information.
    pub private_key: PrivateKey,
    /// The public key.
    #[serde(rename = "publicKey")]
    pub public_key: Vec<u8>,
    /// Address generated from the public key.
    pub address: String,
}

/// Key share variants by protocol.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateKey {
    /// Key share for the GG20 protocol.
    GG20(gg20::KeyShare),
}

impl From<gg20::KeyShare> for KeyShare {
    fn from(local_key: gg20::KeyShare) -> Self {
        let public_key =
            local_key.public_key().to_bytes(false).to_vec();
        Self {
            private_key: PrivateKey::GG20(local_key),
            address: mpc_driver::address(&public_key),
            public_key,
        }
    }
}

/// Server options.
#[derive(Serialize, Deserialize)]
pub struct ServerOptions {
    /// URL for the server.
    pub server_url: String,
    /// Server public key.
    pub server_public_key: Vec<u8>,
}

/// Options used for distributed key generation.
#[derive(Serialize, Deserialize)]
pub struct SessionOptions {
    /// MPC protocol.
    pub protocol: Protocol,
    /// Keypair for the participant.
    pub keypair: Keypair,
    /// Session identifier.
    pub session_id: SessionId,
    /// Server options.
    pub server: ServerOptions,
    /// Parameters for key generation.
    pub parameters: Parameters,
}