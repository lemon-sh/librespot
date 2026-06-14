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

pub mod apresolve;
pub mod audio_key;
pub mod authentication;
pub mod cache;
pub mod cdn_url;
pub mod channel;
pub mod config;
mod connection;
pub mod date;
#[allow(dead_code)]
pub mod dealer;
pub mod deserialize_with;
#[doc(hidden)]
pub mod diffie_hellman;
pub mod error;
pub mod file_id;
pub mod http_client;
pub mod login5;
pub mod mercury;
pub mod packet;
mod proxytunnel;
pub mod session;
mod socket;
#[allow(dead_code)]
pub mod spclient;
pub mod spotify_id;
pub mod spotify_uri;
pub mod token;
#[doc(hidden)]
pub mod util;
pub mod version;

pub use config::SessionConfig;
pub use error::Error;
pub use file_id::FileId;
pub use session::Session;
pub use spotify_id::SpotifyId;
pub use spotify_uri::SpotifyUri;
