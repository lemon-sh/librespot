//! Lyrics metadata.
//!
//! Provides synchronized and unsynchronized lyrics for tracks,
//! fetched via the [`SpClient`](librespot_core::spclient::SpClient).

use bytes::Bytes;

use librespot_core::{Error, FileId, Session, SpotifyId};

impl Lyrics {
    /// Fetches lyrics for the given track.
    pub async fn get(session: &Session, id: &SpotifyId) -> Result<Self, Error> {
        let spclient = session.spclient();
        let lyrics = spclient.get_lyrics(id).await?;
        Self::try_from(&lyrics)
    }

    /// Fetches lyrics associated with a specific image file.
    pub async fn get_for_image(
        session: &Session,
        id: &SpotifyId,
        image_id: &FileId,
    ) -> Result<Self, Error> {
        let spclient = session.spclient();
        let lyrics = spclient.get_lyrics_for_image(id, image_id).await?;
        Self::try_from(&lyrics)
    }
}

impl TryFrom<&Bytes> for Lyrics {
    type Error = Error;

    fn try_from(lyrics: &Bytes) -> Result<Self, Self::Error> {
        serde_json::from_slice(lyrics).map_err(Into::into)
    }
}

/// Synchronized or unsynchronized lyrics for a track.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    /// Color theme for displaying lyrics.
    pub colors: Colors,
    /// Whether a vocal-removal version is available.
    pub has_vocal_removal: bool,
    /// The lyrics content.
    pub lyrics: LyricsInner,
}

/// Color theme for displaying lyrics.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    /// Background color as an integer (ARGB).
    pub background: i32,
    /// Highlight text color as an integer (ARGB).
    pub highlight_text: i32,
    /// Text color as an integer (ARGB).
    pub text: i32,
}

/// The lyrics content with provider information.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsInner {
    // TODO: 'alternatives' field as an array but I don't know what it's meant for
    /// Whether to use a dense typeface for display.
    pub is_dense_typeface: bool,
    /// Whether the lyrics use a right-to-left language.
    pub is_rtl_language: bool,
    /// The language of the lyrics (e.g., `"en"`).
    pub language: String,
    /// The individual lines of lyrics.
    pub lines: Vec<Line>,
    /// The lyrics provider identifier.
    pub provider: String,
    /// The display name of the lyrics provider.
    pub provider_display_name: String,
    /// The provider-specific lyrics identifier.
    pub provider_lyrics_id: String,
    /// The URI for synchronized lyrics, if available.
    pub sync_lyrics_uri: String,
    /// Whether the lyrics are synchronized (timed) or unsynchronized.
    pub sync_type: SyncType,
}

/// Whether lyrics are synchronized (timed) with the audio.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SyncType {
    /// Lyrics are not synchronized with timestamps.
    Unsynced,
    /// Lyrics are synchronized line-by-line.
    LineSynced,
}

/// A single line of lyrics with optional timing information.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    /// Start time in milliseconds (as a string).
    pub start_time_ms: String,
    /// End time in milliseconds (as a string).
    pub end_time_ms: String,
    /// The lyrics text for this line.
    pub words: String,
    // TODO: 'syllables' array
}
