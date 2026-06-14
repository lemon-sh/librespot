//! Track metadata.
//!
//! A [`Track`] represents a Spotify track with its name, artists, album,
//! duration, file formats, and other properties.

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use uuid::Uuid;

use crate::{
    Album, Metadata, RequestResult,
    artist::{Artists, ArtistsWithRole},
    audio::file::AudioFiles,
    availability::Availabilities,
    content_rating::ContentRatings,
    external_id::ExternalIds,
    restriction::Restrictions,
    sale_period::SalePeriods,
    util::{impl_deref_wrapped, impl_try_from_repeated},
};

use librespot_core::{Error, Session, SpotifyUri, date::Date};
use librespot_protocol as protocol;

/// A Spotify track with its full metadata.
#[derive(Debug, Clone)]
pub struct Track {
    /// The Spotify URI of the track.
    pub id: SpotifyUri,
    /// The track name.
    pub name: String,
    /// The album this track belongs to.
    pub album: Album,
    /// The track's performing artists.
    pub artists: Artists,
    /// The track number within its disc.
    pub number: i32,
    /// The disc number within the album.
    pub disc_number: i32,
    /// Duration in milliseconds.
    pub duration: i32,
    /// Popularity score (0–100).
    pub popularity: i32,
    /// Whether the track contains explicit content.
    pub is_explicit: bool,
    /// External identifiers (e.g., ISRC).
    pub external_ids: ExternalIds,
    /// Geographic restrictions.
    pub restrictions: Restrictions,
    /// Available audio file formats.
    pub files: AudioFiles,
    /// Alternative track URIs if this track is unavailable.
    pub alternatives: Tracks,
    /// Sale periods for this track.
    pub sale_periods: SalePeriods,
    /// Preview audio files.
    pub previews: AudioFiles,
    /// Tags associated with the track.
    pub tags: Vec<String>,
    /// The earliest timestamp at which this track becomes live.
    pub earliest_live_timestamp: Date,
    /// Whether lyrics are available for this track.
    pub has_lyrics: bool,
    /// Availability information.
    pub availability: Availabilities,
    /// The licensor UUID.
    pub licensor: Uuid,
    /// Language tags for the track's performance.
    pub language_of_performance: Vec<String>,
    /// Content ratings by country.
    pub content_ratings: ContentRatings,
    /// The original title before any version suffix.
    pub original_title: String,
    /// The version title (e.g., "Deluxe Edition").
    pub version_title: String,
    /// Artists with their roles (e.g., main, featured).
    pub artists_with_role: ArtistsWithRole,
}

/// A list of track URIs.
#[derive(Debug, Clone, Default)]
pub struct Tracks(pub Vec<SpotifyUri>);

impl_deref_wrapped!(Tracks, Vec<SpotifyUri>);

#[async_trait]
impl Metadata for Track {
    type Message = protocol::metadata::Track;

    async fn request(session: &Session, track_uri: &SpotifyUri) -> RequestResult {
        let SpotifyUri::Track { .. } = track_uri else {
            return Err(Error::invalid_argument("track_uri"));
        };

        session.spclient().get_track_metadata(track_uri).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyUri) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Track {
    type Error = librespot_core::Error;
    fn try_from(track: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: track.try_into()?,
            name: track.name().to_owned(),
            album: track.album.get_or_default().try_into()?,
            artists: track.artist.as_slice().try_into()?,
            number: track.number(),
            disc_number: track.disc_number(),
            duration: track.duration(),
            popularity: track.popularity(),
            is_explicit: track.explicit(),
            external_ids: track.external_id.as_slice().into(),
            restrictions: track.restriction.as_slice().into(),
            files: track.file.as_slice().into(),
            alternatives: track.alternative.as_slice().try_into()?,
            sale_periods: track.sale_period.as_slice().try_into()?,
            previews: track.preview.as_slice().into(),
            tags: track.tags.to_vec(),
            earliest_live_timestamp: Date::from_timestamp_ms(track.earliest_live_timestamp())?,
            has_lyrics: track.has_lyrics(),
            availability: track.availability.as_slice().try_into()?,
            licensor: Uuid::from_slice(track.licensor.uuid()).unwrap_or_else(|_| Uuid::nil()),
            language_of_performance: track.language_of_performance.to_vec(),
            content_ratings: track.content_rating.as_slice().into(),
            original_title: track.original_title().to_owned(),
            version_title: track.version_title().to_owned(),
            artists_with_role: track.artist_with_role.as_slice().try_into()?,
        })
    }
}

impl_try_from_repeated!(<Track as Metadata>::Message, Tracks);
