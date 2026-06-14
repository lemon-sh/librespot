//! An open source client library for Spotify, with support for Spotify Connect.
//!
//! # Crate overview
//!
//! | Crate | Role |
//! |---|---|
//! | [`librespot_core`] | Session, authentication, and server communication |
//! | [`librespot_audio`] | Encrypted audio stream fetching and decryption |
//! | [`librespot_metadata`] | Track, album, artist, and other metadata types |
//! | [`librespot_playback`] | Audio decoding, backends, mixing, and normalisation |
//! | [`librespot_connect`] | Spotify Connect protocol for remote device control |
//! | [`librespot_discovery`] | mDNS/DNS-SD advertisement for local network discovery |
//! | [`librespot_oauth`] | OAuth PKCE flow for obtaining access tokens |
//! | [`librespot_protocol`] | Auto-generated protobuf bindings for Spotify's protocol |
//!
//! # Architecture
//!
//! ```text
//! Discovery ──► Session ──► Player ──► Sink (audio backend)
//!                    │          │
//!                    ▼          ▼
//!              SpClient    Metadata / AudioFile
//! ```
//!
//! A typical usage:
//!
//! 1. Obtain [`librespot_core::Credentials`] via [`librespot_discovery`] or [`librespot_oauth`].
//! 2. Create a [`librespot_core::Session`] and call [`Session::connect`](`librespot_core::Session::connect`).
//! 3. Hand the session to a [`Player`](librespot_playback::player::Player), or use
//!    [`Spirc`](librespot_connect::Spirc) for Spotify Connect control.
//!
//! [`librespot_core::Credentials`]: librespot_core::authentication::Credentials
//! [`Session::connect`]: librespot_core::Session::connect

#![crate_name = "librespot"]

pub use librespot_audio as audio;
pub use librespot_connect as connect;
pub use librespot_core as core;
pub use librespot_discovery as discovery;
pub use librespot_metadata as metadata;
pub use librespot_oauth as oauth;
pub use librespot_playback as playback;
pub use librespot_protocol as protocol;
