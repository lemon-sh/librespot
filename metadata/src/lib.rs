//! Spotify metadata types (tracks, albums, artists, episodes, shows, lyrics).
//!
//! This crate provides the [`Metadata`] trait and concrete types for fetching
//! and parsing Spotify's protobuf-encoded metadata.
//!
//! # Usage
//!
//! ```rust,no_run
//! # use librespot_core::Session;
//! # use librespot_metadata::{Track, Metadata};
//! # async fn example(session: Session) -> Result<(), librespot_core::Error> {
//! let uri = "spotify:track:4uLU6hMCjMI75M1A2tKUQC".parse().unwrap();
//! let track = Track::get(&session, &uri).await?;
//! println!("{} by {:?}", track.name, track.artists);
//! # Ok(())
//! # }
//! ```
//!
//! The [`AudioItem`](audio::AudioItem) type provides a unified representation
//! for tracks and episodes, used by the playback pipeline.

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use protobuf::Message;

use librespot_core::{Error, Session, SpotifyUri};

pub mod album;
pub mod artist;
pub mod audio;
pub mod availability;
pub mod content_rating;
pub mod copyright;
pub mod episode;
pub mod error;
pub mod external_id;
pub mod image;
pub mod lyrics;
pub mod playlist;
mod request;
pub mod restriction;
pub mod sale_period;
pub mod show;
pub mod track;
mod util;
pub mod video;

pub use error::MetadataError;
use request::RequestResult;

pub use album::Album;
pub use artist::Artist;
pub use episode::Episode;
pub use lyrics::Lyrics;
pub use playlist::Playlist;
pub use show::Show;
pub use track::Track;

/// Trait for fetching and parsing Spotify metadata.
///
/// Implementors can be fetched from the server via [`Metadata::get`].
/// The trait abstracts over the protobuf wire format: each type specifies
/// its corresponding protobuf `Message` type and
/// provides a [`parse`](Metadata::parse) method to convert it.
#[async_trait]
pub trait Metadata: Send + Sized + 'static {
    /// The protobuf message type used for deserialization.
    type Message: protobuf::Message + std::fmt::Debug;

    /// Fetches the raw protobuf bytes from the server.
    async fn request(session: &Session, id: &SpotifyUri) -> RequestResult;

    /// Fetches and parses the metadata for the given resource URI.
    async fn get(session: &Session, id: &SpotifyUri) -> Result<Self, Error> {
        let response = Self::request(session, id).await?;
        let msg = Self::Message::parse_from_bytes(&response)?;
        trace!("Received metadata: {msg:#?}");
        Self::parse(&msg, id)
    }

    /// Parses a protobuf message into this metadata type.
    fn parse(msg: &Self::Message, _: &SpotifyUri) -> Result<Self, Error>;
}
