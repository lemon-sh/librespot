//! Audio decoding, playback management, and audio backends.
//!
//! This crate provides the `Player` for loading and playing tracks, and the
//! audio pipeline: decoder → converter → mixer → sink.
//!
//! # Key types
//!
//! - `Player` — loads tracks, manages playback state, emits `PlayerEvent`s
//! - [`Sink`](audio_backend::Sink) — trait for audio output backends
//! - [`Mixer`](mixer::Mixer) — trait for volume control
//! - [`AudioDecoder`](decoder::AudioDecoder) — trait for audio decoders
//! - `PlayerConfig` — playback configuration
//!
//! # Audio pipeline
//!
//! ```text
//! AudioFile → AudioDecrypt → SymphoniaDecoder → Converter → Mixer → Sink
//! ```

#[macro_use]
extern crate log;

use librespot_audio as audio;
use librespot_core as core;
use librespot_metadata as metadata;

pub mod audio_backend;
pub mod config;
pub mod convert;
pub mod decoder;
pub mod dither;
mod local_file;
pub mod mixer;
pub mod player;
mod symphonia_util;

/// Standard sample rate for Spotify audio (44.1 kHz).
pub const SAMPLE_RATE: u32 = 44100;
/// Number of audio channels (stereo).
pub const NUM_CHANNELS: u8 = 2;
/// Total samples per second across all channels.
pub const SAMPLES_PER_SECOND: u32 = SAMPLE_RATE * NUM_CHANNELS as u32;
/// Audio pages per millisecond.
pub const PAGES_PER_MS: f64 = SAMPLE_RATE as f64 / 1000.0;
/// Milliseconds per audio page.
pub const MS_PER_PAGE: f64 = 1000.0 / SAMPLE_RATE as f64;
