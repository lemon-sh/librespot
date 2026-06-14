use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use protobuf::Message;
use thiserror::Error;

use crate::{Error, packet::PacketType, protocol};

/// Mercury request method.
#[derive(Debug, PartialEq, Eq)]
pub enum MercuryMethod {
    /// A GET request.
    Get,
    /// A SUB (subscribe) request.
    Sub,
    /// An UNSUB (unsubscribe) request.
    Unsub,
    /// A SEND request.
    Send,
}

/// A Mercury request message.
#[derive(Debug)]
pub struct MercuryRequest {
    /// The request method.
    pub method: MercuryMethod,
    /// The target URI.
    pub uri: String,
    /// Optional content type.
    pub content_type: Option<String>,
    /// Request payload parts.
    pub payload: Vec<Vec<u8>>,
}

/// A Mercury response message.
#[derive(Debug, Clone)]
pub struct MercuryResponse {
    /// The response URI.
    pub uri: String,
    /// The HTTP-style status code.
    pub status_code: i32,
    /// Response payload parts.
    pub payload: Vec<Vec<u8>>,
}

/// Errors that can occur during Mercury operations.
#[derive(Debug, Error)]
pub enum MercuryError {
    /// The callback receiver was disconnected.
    #[error("callback receiver was disconnected")]
    Channel,
    /// Error handling a specific packet type.
    #[error("error handling packet type: {0:?}")]
    Command(PacketType),
    /// Error handling a Mercury response.
    #[error("error handling Mercury response: {0:?}")]
    Response(MercuryResponse),
}

impl From<MercuryError> for Error {
    fn from(err: MercuryError) -> Self {
        match err {
            MercuryError::Channel => Error::aborted(err),
            MercuryError::Command(_) => Error::unimplemented(err),
            MercuryError::Response(_) => Error::unavailable(err),
        }
    }
}

impl std::fmt::Display for MercuryMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            MercuryMethod::Get => "GET",
            MercuryMethod::Sub => "SUB",
            MercuryMethod::Unsub => "UNSUB",
            MercuryMethod::Send => "SEND",
        };
        write!(f, "{s}")
    }
}

impl MercuryMethod {
    /// Returns the corresponding packet type for this method.
    pub fn command(&self) -> PacketType {
        use PacketType::*;
        match *self {
            MercuryMethod::Get | MercuryMethod::Send => MercuryReq,
            MercuryMethod::Sub => MercurySub,
            MercuryMethod::Unsub => MercuryUnsub,
        }
    }
}

impl MercuryRequest {
    /// Encodes this request into a binary packet with the given sequence number.
    pub fn encode(&self, seq: &[u8]) -> Result<Vec<u8>, Error> {
        let mut packet = Vec::new();
        packet.write_u16::<BigEndian>(seq.len() as u16)?;
        packet.write_all(seq)?;
        packet.write_u8(1)?; // Flags: FINAL
        packet.write_u16::<BigEndian>(1 + self.payload.len() as u16)?; // Part count

        let mut header = protocol::mercury::Header::new();
        header.set_uri(self.uri.clone());
        header.set_method(self.method.to_string());

        if let Some(ref content_type) = self.content_type {
            header.set_content_type(content_type.clone());
        }

        packet.write_u16::<BigEndian>(header.compute_size() as u16)?;
        header.write_to_writer(&mut packet)?;

        for p in &self.payload {
            packet.write_u16::<BigEndian>(p.len() as u16)?;
            packet.write_all(p)?;
        }

        Ok(packet)
    }
}
