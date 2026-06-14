//! Encrypted audio stream fetching and decryption.
//!
//! This crate handles the pipeline from Spotify's CDN to raw PCM audio:
//!
//! 1. [`AudioFile`] fetches encrypted audio chunks via HTTP range requests
//! 2. [`AudioDecrypt`] decrypts the data using AES-128-CTR with an [`AudioKey`](librespot_core::audio_key::AudioKey)
//!
//! The [`StreamLoaderController`] provides flow control for the streaming pipeline,
//! allowing the decoder to signal which ranges need to be fetched.

#[macro_use]
extern crate log;

mod decrypt;
mod fetch;

mod range_set;

pub use decrypt::AudioDecrypt;
pub use fetch::{AudioFetchParams, AudioFile, AudioFileError, StreamLoaderController};
