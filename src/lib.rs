#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::io;

use proto::Packet;
use thiserror::Error;

pub mod proto;
pub mod client;
pub mod commands;

pub use client::MDCSession;
pub use commands::DISPLAY_BROADCAST;
pub use client::DisplayControl;

/// General error that can occur during communication with MDC server
#[derive(Debug, Error)]
pub enum Error {
    /// IO Error
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    /// Failed to parse packet
    #[error("Invalid packet: {0}")]
    InvalidPacket(#[from] proto::Error),
    /// Stream ended or was closed before a packet ended
    #[error("Stream ended before sending full packet")]
    UnexpectedEndOfStream,
    /// Received a packet that was unexpected
    #[error("Unexpected response packet")]
    UnexpectedResponse(Packet),
    /// Server responded with NACK
    #[error("Server responded with NACK")]
    Nack(Packet)
}