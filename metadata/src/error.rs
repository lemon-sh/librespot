use std::fmt::Debug;
use thiserror::Error;

/// Errors that can occur when fetching or parsing Spotify metadata.
#[derive(Debug, Error)]
pub enum MetadataError {
    /// The server returned an empty response.
    #[error("empty response")]
    Empty,
    /// The audio item is not available for playback.
    #[error("audio item is non-playable when it should be")]
    NonPlayable,
    /// The audio item has an invalid duration.
    #[error("audio item duration can not be: {0}")]
    InvalidDuration(i32),
    /// The track contains explicit content, which the client setting filters out.
    #[error("track is marked as explicit, which client setting forbids")]
    ExplicitContentFiltered,
}
