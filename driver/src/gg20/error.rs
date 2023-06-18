use thiserror::Error;
use multi_party_ecdsa::protocols::{
    multi_party_ecdsa::gg_2020::state_machine::{keygen, sign},
};

/// Errors generated by the protocol.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated when the user's public key is not in the
    /// list of session participants.
    #[error("public key {0} is not a session participant")]
    NotSessionParticipant(String),

    /// Error generated the local key used for signing does not
    /// exist in the list of participants.
    #[error("local key index not found in list of participants")]
    LocalKeyNotParticipant,

    /// Signature verification failed.
    #[error("failed to verify generated signature")]
    VerifySignature,

    /// Key generation error.
    #[error(transparent)]
    Keygen(#[from] keygen::Error),

    /// Signing error.
    #[error(transparent)]
    SignOffline(#[from] sign::Error),

    /// Signing error.
    #[error(transparent)]
    SignManual(#[from] sign::SignError),

    /// Driver library error.
    #[error(transparent)]
    Driver(#[from] Box<crate::Error>),
}
