//! Spotify Connect protocol implementation.
//!
//! This crate handles the Connect protocol for remote device control:
//! device registration, remote commands, context resolution, queue management,
//! and state synchronization with Spotify's servers.
//!
//! # Key types
//!
//! - [`Spirc`] — handle for controlling the Connect device
//! - [`ConnectConfig`] — device configuration (name, type, volume)
//! - [`LoadRequest`] — request to load a context for playback
//!
//! # Usage
//!
//! ```rust,no_run
//! # use std::sync::Arc;
//! # use librespot_core::Session;
//! # use librespot_playback::player::Player;
//! # use librespot_playback::mixer::Mixer;
//! # use librespot_connect::{Spirc, ConnectConfig};
//! # use librespot_core::authentication::Credentials;
//! # async fn example(session: Session, player: Arc<Player>, mixer: Arc<dyn Mixer>) -> Result<(), librespot_core::Error> {
//! let config = ConnectConfig::default();
//! let creds = Credentials::with_password("user", "pass");
//! let (spirc, task) = Spirc::new(config, session, creds, player, mixer).await?;
//! tokio::spawn(task);
//! spirc.play()?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

#[macro_use]
extern crate log;

use librespot_core as core;
use librespot_playback as playback;
use librespot_protocol as protocol;

mod context_resolver;
mod model;
mod shuffle_vec;
mod spirc;
mod state;

pub use model::*;
pub use spirc::*;
pub use state::*;
