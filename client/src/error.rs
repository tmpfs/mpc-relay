use mpc_relay_protocol::{
    http::StatusCode, RequestMessage, ResponseMessage,
};
use thiserror::Error;

/// Errors generated by the relay client.
#[derive(Debug, Error)]
pub enum Error {
    /// Error generated attempting to connect to
    /// the websocket server when the response is
    /// not a 101 switching protocols status code.
    #[error("{0} {1}")]
    ConnectError(StatusCode, String),

    /// Error generated by the server.
    #[error("{0} {1}")]
    ServerError(StatusCode, String),

    /// Error generated if the client expects a reply but none was received.
    #[error("server did not reply")]
    NoReply,

    /// Error generated attempting to handshake with a peer that
    /// already exists.
    #[error("peer already exists")]
    PeerAlreadyExists,

    /// Error generated attempting to handshake with a peer that
    /// already exists.
    #[error(
        "peer already exists, maybe peers are racing to connect"
    )]
    PeerAlreadyExistsMaybeRace,

    /// Error generated when a peer could not be found.
    #[error(r#"peer "{0}" not found "#)]
    PeerNotFound(String),

    /// Error generated when a node expects to be in the handshake
    /// protocol state.
    #[error("not handshake protocol state")]
    NotHandshakeState,

    /// Error generated when a node expects to be in the transport
    /// protocol state.
    #[error("not transport protocol state")]
    NotTransportState,

    /// Error generated when the wrong type of message is encountered
    /// during a peer to peer handshake.
    #[error("invalid peer handshake message")]
    InvalidPeerHandshakeMessage,

    /// Error generated by input/output.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Error generated by the protocol library.
    #[error(transparent)]
    Protocol(#[from] mpc_relay_protocol::Error),

    /// Error generated by the noise protocol library.
    #[error(transparent)]
    Snow(#[from] mpc_relay_protocol::snow::error::Error),

    /// Error generated serializing or deserializing JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    /// Error generated by the client websocket library.
    #[error(transparent)]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// Error generated sending a request over a channel.
    #[error(transparent)]
    RequestMpscSend(
        #[from] tokio::sync::mpsc::error::SendError<RequestMessage>,
    ),

    /// Error generated sending a response over a channel.
    #[error(transparent)]
    ResponseMpscSend(
        #[from] tokio::sync::mpsc::error::SendError<ResponseMessage>,
    ),
}
