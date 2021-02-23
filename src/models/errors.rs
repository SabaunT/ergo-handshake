use std::io;
use std::string::FromUtf8Error;

use thiserror::Error;

use super::features::Features;
use super::peer_addr::PeerAddr;
use super::short_string::ShortString;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Can't create ShortString from buffer with length {0}. Should be {}", ShortString::MAX_SIZE)]
    InvalidShortStringLength(usize),
    #[error("Received invalid data: {0}")]
    InvalidUtf8Buffer(#[source] FromUtf8Error),
    #[error(
        "Can't create HSPeerAddr from buffer with length {0}. Should be {} or {}",
        PeerAddr::SIZE_IPv4_SOCKET,
        PeerAddr::SIZE_IPv6_SOCKET
    )]
    InvalidPeerAddrLength(usize),
    #[error("Can't create Features from features sequence with length {0}. Should be {}", Features::MAX_SIZE)]
    InvalidFeaturesLength(usize),
    #[error("Model can't be serialized: {0}")]
    CannotSerializeData(#[source] io::Error),
}

// todo-minor ERRORS HIERARCHY IS WASTED!