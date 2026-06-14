//! Audio decoders.
//!
//! Decoders convert compressed audio (Ogg Vorbis, FLAC, MP3) into raw samples.
//! The [`AudioDecoder`] trait abstracts over decoder implementations.
//!
//! Available decoders:
//! - [`SymphoniaDecoder`] — multi-format via Symphonia (default)
//! - `PassthroughDecoder` — raw Ogg passthrough (feature-gated)
use std::ops::Deref;

use thiserror::Error;

#[cfg(feature = "passthrough-decoder")]
mod passthrough_decoder;
#[cfg(feature = "passthrough-decoder")]
pub use passthrough_decoder::PassthroughDecoder;

mod symphonia_decoder;
pub use symphonia_decoder::SymphoniaDecoder;

/// Errors that can occur during audio decoding.
#[derive(Error, Debug)]
pub enum DecoderError {
    /// An error from the passthrough decoder.
    #[error("Passthrough Decoder Error: {0}")]
    PassthroughDecoder(String),
    /// An error from the Symphonia decoder.
    #[error("Symphonia Decoder Error: {0}")]
    SymphoniaDecoder(String),
}

/// Result type for decoder operations.
pub type DecoderResult<T> = Result<T, DecoderError>;

/// Errors that can occur when accessing audio packet data.
#[derive(Error, Debug)]
pub enum AudioPacketError {
    /// Attempted to get raw bytes from a samples packet.
    #[error("Decoder Raw Error: Can't return Raw on Samples")]
    Raw,
    /// Attempted to get samples from a raw bytes packet.
    #[error("Decoder Samples Error: Can't return Samples on Raw")]
    Samples,
}

/// Result type for audio packet operations.
pub type AudioPacketResult<T> = Result<T, AudioPacketError>;

/// A decoded audio packet containing either samples or raw bytes.
pub enum AudioPacket {
    /// Audio samples as f64 values (normalized to -1.0..1.0).
    Samples(Vec<f64>),
    /// Raw audio bytes (used for passthrough mode).
    Raw(Vec<u8>),
}

impl AudioPacket {
    /// Returns the audio samples, or an error if this is a raw packet.
    #[inline]
    pub fn samples(&self) -> AudioPacketResult<&[f64]> {
        match self {
            AudioPacket::Samples(s) => Ok(s),
            AudioPacket::Raw(_) => Err(AudioPacketError::Raw),
        }
    }

    /// Returns the raw bytes, or an error if this is a samples packet.
    #[inline]
    pub fn raw(&self) -> AudioPacketResult<&[u8]> {
        match self {
            AudioPacket::Raw(d) => Ok(d),
            AudioPacket::Samples(_) => Err(AudioPacketError::Samples),
        }
    }

    /// Returns `true` if the packet contains no data.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            AudioPacket::Samples(s) => s.is_empty(),
            AudioPacket::Raw(d) => d.is_empty(),
        }
    }
}

/// Position information for a decoded audio packet.
#[derive(Debug, Clone)]
pub struct AudioPacketPosition {
    /// The position in milliseconds within the track.
    pub position_ms: u32,
    /// Whether this packet's data was skipped due to a decode error.
    pub skipped: bool,
}

impl Deref for AudioPacketPosition {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.position_ms
    }
}

/// Trait for audio decoders.
pub trait AudioDecoder {
    /// Seeks to the given position in milliseconds. Returns the actual position.
    fn seek(&mut self, position_ms: u32) -> Result<u32, DecoderError>;
    /// Returns the next decoded audio packet, or `None` at end of stream.
    fn next_packet(&mut self) -> DecoderResult<Option<(AudioPacketPosition, AudioPacket)>>;
}

impl From<DecoderError> for librespot_core::error::Error {
    fn from(err: DecoderError) -> Self {
        librespot_core::error::Error::aborted(err)
    }
}

impl From<symphonia::core::errors::Error> for DecoderError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        Self::SymphoniaDecoder(err.to_string())
    }
}
