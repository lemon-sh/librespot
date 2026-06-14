use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    Metadata,
    request::RequestResult,
    util::{impl_deref_wrapped, impl_from_repeated_copy, impl_try_from_repeated},
};

use super::{
    attribute::PlaylistAttributes, diff::PlaylistDiff, item::PlaylistItemList,
    permission::Capabilities,
};

use librespot_core::{Error, Session, SpotifyUri, date::Date, spotify_id::SpotifyId};
use librespot_protocol as protocol;
use protocol::playlist4_external::GeoblockBlockingType as Geoblock;

/// Geographic blocking types for a playlist.
#[derive(Debug, Clone, Default)]
pub struct Geoblocks(Vec<Geoblock>);

impl_deref_wrapped!(Geoblocks, Vec<Geoblock>);

/// A Spotify playlist with its contents and metadata.
#[derive(Debug, Clone)]
pub struct Playlist {
    /// The Spotify URI of the playlist.
    pub id: SpotifyUri,
    /// The playlist revision (opaque binary data).
    pub revision: Vec<u8>,
    /// The number of items in the playlist.
    pub length: i32,
    /// The playlist attributes (name, description, etc.).
    pub attributes: PlaylistAttributes,
    /// The playlist items.
    pub contents: PlaylistItemList,
    /// Optional diff from a previous revision.
    pub diff: Option<PlaylistDiff>,
    /// Optional sync result diff.
    pub sync_result: Option<PlaylistDiff>,
    /// Resulting revisions after operations.
    pub resulting_revisions: Playlists,
    /// Whether the playlist has multiple heads.
    pub has_multiple_heads: bool,
    /// Whether the playlist is up to date.
    pub is_up_to_date: bool,
    /// Nonce values.
    pub nonces: Vec<i64>,
    /// The playlist timestamp.
    pub timestamp: Date,
    /// Whether abuse reporting is enabled.
    pub has_abuse_reporting: bool,
    /// The user's capabilities for this playlist.
    pub capabilities: Capabilities,
    /// Geographic blocking restrictions.
    pub geoblocks: Geoblocks,
}

/// A list of playlist [`SpotifyId`]s.
#[derive(Debug, Clone, Default)]
pub struct Playlists(pub Vec<SpotifyId>);

impl_deref_wrapped!(Playlists, Vec<SpotifyId>);

/// Raw selected list content from the Spotify API, decorated with owner info.
#[derive(Debug, Clone)]
pub struct SelectedListContent {
    /// The playlist revision.
    pub revision: Vec<u8>,
    /// The number of items.
    pub length: i32,
    /// The playlist attributes.
    pub attributes: PlaylistAttributes,
    /// The playlist items.
    pub contents: PlaylistItemList,
    /// Optional diff from a previous revision.
    pub diff: Option<PlaylistDiff>,
    /// Optional sync result diff.
    pub sync_result: Option<PlaylistDiff>,
    /// Resulting revisions after operations.
    pub resulting_revisions: Playlists,
    /// Whether the playlist has multiple heads.
    pub has_multiple_heads: bool,
    /// Whether the playlist is up to date.
    pub is_up_to_date: bool,
    /// Nonce values.
    pub nonces: Vec<i64>,
    /// The playlist timestamp.
    pub timestamp: Date,
    /// The owner's username.
    pub owner_username: String,
    /// Whether abuse reporting is enabled.
    pub has_abuse_reporting: bool,
    /// The user's capabilities for this playlist.
    pub capabilities: Capabilities,
    /// Geographic blocking restrictions.
    pub geoblocks: Geoblocks,
}

impl Playlist {
    /// Returns an iterator over all track URIs in the playlist.
    pub fn tracks(&self) -> impl ExactSizeIterator<Item = &SpotifyUri> {
        let tracks = self.contents.items.iter().map(|item| &item.id);

        let length = tracks.len();
        let expected_length = self.length as usize;
        if length != expected_length {
            warn!("Got {length} tracks, but the list should contain {expected_length} tracks.",);
        }

        tracks
    }

    /// Returns the playlist name.
    pub fn name(&self) -> &str {
        &self.attributes.name
    }
}

#[async_trait]
impl Metadata for Playlist {
    type Message = protocol::playlist4_external::SelectedListContent;

    async fn request(session: &Session, playlist_uri: &SpotifyUri) -> RequestResult {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = playlist_uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        session.spclient().get_playlist(playlist_id).await
    }

    fn parse(msg: &Self::Message, uri: &SpotifyUri) -> Result<Self, Error> {
        let SpotifyUri::Playlist {
            id: playlist_id, ..
        } = uri
        else {
            return Err(Error::invalid_argument("playlist_uri"));
        };

        // the playlist proto doesn't contain the id so we decorate it
        let playlist = SelectedListContent::try_from(msg)?;

        let new_uri = SpotifyUri::Playlist {
            id: *playlist_id,
            user: Some(playlist.owner_username),
        };

        Ok(Self {
            id: new_uri,
            revision: playlist.revision,
            length: playlist.length,
            attributes: playlist.attributes,
            contents: playlist.contents,
            diff: playlist.diff,
            sync_result: playlist.sync_result,
            resulting_revisions: playlist.resulting_revisions,
            has_multiple_heads: playlist.has_multiple_heads,
            is_up_to_date: playlist.is_up_to_date,
            nonces: playlist.nonces,
            timestamp: playlist.timestamp,
            has_abuse_reporting: playlist.has_abuse_reporting,
            capabilities: playlist.capabilities,
            geoblocks: playlist.geoblocks,
        })
    }
}

impl TryFrom<&<Playlist as Metadata>::Message> for SelectedListContent {
    type Error = librespot_core::Error;
    fn try_from(playlist: &<Playlist as Metadata>::Message) -> Result<Self, Self::Error> {
        let timestamp = playlist.timestamp();
        let timestamp = if timestamp > 9295169800000 {
            // timestamp is way out of range for milliseconds. Some seem to be in microseconds?
            // Observed on playlists where:
            //   format: "artist-mix-reader"
            //   format_attributes {
            //     key: "mediaListConfig"
            //     value: "spotify:medialistconfig:artist-seed-mix:default_v18"
            //   }
            warn!("timestamp is very large; assuming it's in microseconds");
            timestamp / 1000
        } else {
            timestamp
        };
        let timestamp = Date::from_timestamp_ms(timestamp)?;

        Ok(Self {
            revision: playlist.revision().to_owned(),
            length: playlist.length(),
            attributes: playlist.attributes.get_or_default().try_into()?,
            contents: playlist.contents.get_or_default().try_into()?,
            diff: playlist.diff.as_ref().map(TryInto::try_into).transpose()?,
            sync_result: playlist
                .sync_result
                .as_ref()
                .map(TryInto::try_into)
                .transpose()?,
            resulting_revisions: Playlists(
                playlist
                    .resulting_revisions
                    .iter()
                    .map(std::convert::TryInto::try_into)
                    .collect::<Result<Vec<SpotifyId>, Error>>()?,
            ),
            has_multiple_heads: playlist.multiple_heads(),
            is_up_to_date: playlist.up_to_date(),
            nonces: playlist.nonces.clone(),
            timestamp,
            owner_username: playlist.owner_username().to_owned(),
            has_abuse_reporting: playlist.abuse_reporting_enabled(),
            capabilities: playlist.capabilities.get_or_default().into(),
            geoblocks: Geoblocks(
                playlist
                    .geoblock
                    .iter()
                    .map(protobuf::EnumOrUnknown::enum_value_or_default)
                    .collect(),
            ),
        })
    }
}

impl_from_repeated_copy!(Geoblock, Geoblocks);
impl_try_from_repeated!(Vec<u8>, Playlists);
