//! GG20 message signing.
use curv::{
    arithmetic::Converter, elliptic::curves::Secp256k1, BigInt,
};
use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::{
    party_i::{verify, SignatureRecid},
    state_machine::{
        keygen::LocalKey,
        sign::{
            CompletedOfflineStage, OfflineProtocolMessage,
            OfflineStage, PartialSignature, SignManual,
        },
    },
};

use super::{Error, Result};
use crate::{ProtocolDriver, RoundMsg};
use round_based::{Msg, StateMachine};
use serde::{Deserialize, Serialize};

type Message = Msg<<OfflineStage as StateMachine>::MessageBody>;

/// Signature generated by a signer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    /// The generated ECDSA signature.
    pub signature: SignatureRecid,
    /// The public key for the signature.
    pub public_key: Vec<u8>,
    /// Address generated from the public key.
    pub address: String,
}

/// Result from driving the offline signing stage.
pub struct OfflineResult {
    data: BigInt,
    partial: PartialSignature,
    completed_offline_stage: CompletedOfflineStage,
}

/// Drive the offline signing stage.
pub struct SignOffline {
    inner: OfflineStage,
    message: [u8; 32],
}

impl SignOffline {
    /// Create a signer.
    pub fn new(
        index: u16,
        participants: Vec<u16>,
        local_key: LocalKey<Secp256k1>,
        message: [u8; 32],
    ) -> Result<SignOffline> {
        Ok(SignOffline {
            inner: OfflineStage::new(index, participants, local_key)?,
            message,
        })
    }
}

impl ProtocolDriver for SignOffline {
    type Error = Error;
    type Incoming = Message;
    type Outgoing = RoundMsg<OfflineProtocolMessage>;
    type Output = OfflineResult;

    fn handle_incoming(
        &mut self,
        message: Self::Incoming,
    ) -> Result<()> {
        self.inner.handle_incoming(message)?;
        Ok(())
    }

    fn proceed(&mut self) -> Result<(u16, Vec<Self::Outgoing>)> {
        self.inner.proceed()?;
        let messages = self.inner.message_queue().drain(..).collect();
        let round = self.inner.current_round();
        let messages = RoundMsg::from_round(round, messages);
        Ok((round, messages))
    }

    fn finish(&mut self) -> Result<Self::Output> {
        let completed_offline_stage =
            self.inner.pick_output().unwrap()?;
        let data = BigInt::from_bytes(&self.message);
        let (_sign, partial) = SignManual::new(
            data.clone(),
            completed_offline_stage.clone(),
        )?;
        Ok(OfflineResult {
            data,
            partial,
            completed_offline_stage,
        })
    }
}

/// Drive the online signing stage.
pub struct SignOnline {
    offline: OfflineResult,
    partials: Vec<PartialSignature>,
}

impl SignOnline {
    /// Create a signer.
    pub fn new(
        offline: OfflineResult,
        partials: Vec<PartialSignature>,
    ) -> Result<SignOnline> {
        Ok(Self { offline, partials })
    }
}

impl ProtocolDriver for SignOnline {
    type Error = Error;
    type Incoming = Message;
    type Outgoing = RoundMsg<OfflineProtocolMessage>;
    type Output = Signature;

    fn handle_incoming(
        &mut self,
        _message: Self::Incoming,
    ) -> Result<()> {
        panic!("online signing does not require incoming messages");
    }

    fn proceed(&mut self) -> Result<(u16, Vec<Self::Outgoing>)> {
        panic!("online signing does not send outgoing messages");
    }

    fn finish(&mut self) -> Result<Self::Output> {
        let data = self.offline.data.clone();
        let pk =
            self.offline.completed_offline_stage.public_key().clone();

        let (sign, _partial) = SignManual::new(
            data.clone(),
            self.offline.completed_offline_stage.clone(),
        )?;

        let signature = sign.complete(&self.partials)?;
        verify(&signature, &pk, &data).map_err(|_| Error::VerifySignature)?;

        let public_key = pk.to_bytes(false).to_vec();
        let result = Signature {
            signature,
            address: crate::address(&public_key),
            public_key,
        };

        Ok(result)
    }
}