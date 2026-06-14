//! Podcast episode metadata.
//!
//! An [`Episode`] represents a Spotify podcast episode with its audio files,
//! description, show name, and publication date.

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    Metadata,
    audio::file::AudioFiles,
    availability::Availabilities,
    content_rating::ContentRatings,
    image::Images,
    request::RequestResult,
    restriction::Restrictions,
    util::{impl_deref_wrapped, impl_try_from_repeated},
    video::VideoFiles,
};

use librespot_core::{Error, Session, SpotifyUri, date::Date};

use librespot_protocol as protocol;
pub use protocol::metadata::episode::EpisodeType;

/// A Spotify podcast episode with its full metadata.
#[derive(Debug, Clone)]
pub struct Episode {
    /// The Spotify URI of the episode.
    pub id: SpotifyUri,
    /// The episode name.
    pub name: String,
    /// Duration in milliseconds.
    pub duration: i32,
    /// Available audio files for this episode.
    pub audio: AudioFiles,
    /// The episode description.
    pub description: String,
    /// The episode number within its show.
    pub number: i32,
    /// The publication date.
    pub publish_time: Date,
    /// Cover art images.
    pub covers: Images,
    /// The language of the episode (e.g., `"en"`).
    pub language: String,
    /// Whether the episode contains explicit content.
    pub is_explicit: bool,
    /// The name of the show this episode belongs to.
    pub show_name: String,
    /// Available video files.
    pub videos: VideoFiles,
    /// Video preview clips.
    pub video_previews: VideoFiles,
    /// Audio preview clips.
    pub audio_previews: AudioFiles,
    /// Geographic restrictions.
    pub restrictions: Restrictions,
    /// Freeze-frame images for the episode.
    pub freeze_frames: Images,
    /// Search keywords.
    pub keywords: Vec<String>,
    /// Whether background playback is allowed.
    pub allow_background_playback: bool,
    /// Availability information.
    pub availability: Availabilities,
    /// External URL for the episode.
    pub external_url: String,
    /// The type of episode (full, trailer, etc.).
    pub episode_type: EpisodeType,
    /// Whether the episode contains music and talk content.
    pub has_music_and_talk: bool,
    /// Content ratings by country.
    pub content_rating: ContentRatings,
    /// Whether this episode is an audiobook chapter.
    pub is_audiobook_chapter: bool,
}

/// A list of episode URIs.
#[derive(Debug, Clone, Default)]
pub struct Episodes(pub Vec<SpotifyUri>);

impl_deref_wrapped!(Episodes, Vec<SpotifyUri>);

#[async_trait]
impl Metadata for Episode {
    type Message = protocol::metadata::Episode;

    async fn request(session: &Session, episode_uri: &SpotifyUri) -> RequestResult {
        let SpotifyUri::Episode { .. } = episode_uri else {
            return Err(Error::invalid_argument("episode_uri"));
        };

        session.spclient().get_episode_metadata(episode_uri).await
    }

    fn parse(msg: &Self::Message, _: &SpotifyUri) -> Result<Self, Error> {
        Self::try_from(msg)
    }
}

impl TryFrom<&<Self as Metadata>::Message> for Episode {
    type Error = librespot_core::Error;
    fn try_from(episode: &<Self as Metadata>::Message) -> Result<Self, Self::Error> {
        Ok(Self {
            id: episode.try_into()?,
            name: episode.name().to_owned(),
            duration: episode.duration().to_owned(),
            audio: episode.audio.as_slice().into(),
            description: episode.description().to_owned(),
            number: episode.number(),
            publish_time: episode.publish_time.get_or_default().try_into()?,
            covers: episode.cover_image.image.as_slice().into(),
            language: episode.language().to_owned(),
            is_explicit: episode.explicit().to_owned(),
            show_name: episode.show.name().to_owned(),
            videos: episode.video.as_slice().into(),
            video_previews: episode.video_preview.as_slice().into(),
            audio_previews: episode.audio_preview.as_slice().into(),
            restrictions: episode.restriction.as_slice().into(),
            freeze_frames: episode.freeze_frame.image.as_slice().into(),
            keywords: episode.keyword.to_vec(),
            allow_background_playback: episode.allow_background_playback(),
            availability: episode.availability.as_slice().try_into()?,
            external_url: episode.external_url().to_owned(),
            episode_type: episode.type_(),
            has_music_and_talk: episode.music_and_talk(),
            content_rating: episode.content_rating.as_slice().into(),
            is_audiobook_chapter: episode.is_audiobook_chapter(),
        })
    }
}

impl_try_from_repeated!(<Episode as Metadata>::Message, Episodes);
