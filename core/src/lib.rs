//! Core session management and server communication for librespot.
//!
//! This crate provides the foundational types used by all other librespot crates:
//!
//! - [`Session`] — authenticated connection to Spotify's servers
//! - `Credentials` — login credentials (password, access token, or blob)
//! - [`SpotifyId`] / [`SpotifyUri`] — resource identifiers
//! - [`Error`] — unified error type with gRPC-style error kinds
//!
//! # Session components
//!
//! A [`Session`] lazily initializes sub-managers on first use:
//!
//! | Manager | Protocol | Purpose |
//! |---|---|---|
//! | [`ApResolver`](apresolve::ApResolver) | HTTP | Resolves access point addresses |
//! | [`AudioKeyManager`](audio_key::AudioKeyManager) | Shannon | Fetches AES keys for audio decryption |
//! | [`ChannelManager`](channel::ChannelManager) | Shannon | Binary data streaming channels |
//! | [`MercuryManager`](mercury::MercuryManager) | Shannon | Legacy pub/sub messaging |
//! | [`SpClient`](spclient::SpClient) | HTTP | Spotify Web API (spclient) |
//! | [`TokenProvider`](token::TokenProvider) | HTTP | Access token management |
//! | [`Login5Manager`](login5::Login5Manager) | HTTP | Mobile login protocol |
//!
//! These are created via the `component!` macro, which wraps each in an
//! `Arc<(SessionWeak, Mutex<Inner>)>` pattern with weak session references
//! to prevent reference cycles.

#[macro_use]
extern crate log;

use librespot_protocol as protocol;

#[macro_use]
mod component;

/// Access point resolution for Spotify's servers.
pub mod apresolve;
/// Audio key management for decrypting audio streams.
pub mod audio_key;
/// Credentials for authenticating with Spotify.
pub mod authentication;
/// Persistent caching for credentials, volume, and audio files.
pub mod cache;
/// CDN URL resolution for audio storage.
pub mod cdn_url;
/// Binary data streaming channels.
pub mod channel;
/// Configuration types for [`Session`](crate::Session).
pub mod config;
mod connection;
/// Date and time utilities.
pub mod date;
#[allow(dead_code)]
/// Dealer WebSocket message bus for real-time commands.
pub mod dealer;
/// Custom serde deserialization helpers.
pub mod deserialize_with;
#[doc(hidden)]
pub mod diffie_hellman;
/// Unified error type for librespot.
pub mod error;
/// Audio and image file identifiers.
pub mod file_id;
/// HTTP client with rate limiting and proxy support.
pub mod http_client;
/// Login5 authentication protocol.
pub mod login5;
/// Mercury pub/sub protocol (legacy).
pub mod mercury;
/// Spotify packet type definitions.
pub mod packet;
mod proxytunnel;
/// Session management and server communication.
pub mod session;
mod socket;
#[allow(dead_code)]
/// Spotify Web API client (spclient).
pub mod spclient;
/// Spotify resource identifiers.
pub mod spotify_id;
/// Typed Spotify URIs.
pub mod spotify_uri;
/// Access token management.
pub mod token;
#[doc(hidden)]
pub mod util;
/// Build and protocol version constants.
pub mod version;

pub use config::SessionConfig;
pub use error::Error;
pub use file_id::FileId;
pub use session::Session;
pub use spotify_id::SpotifyId;
pub use spotify_uri::SpotifyUri;
