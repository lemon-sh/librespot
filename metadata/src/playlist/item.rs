use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_try_from_repeated};

use super::{
    attribute::{PlaylistAttributes, PlaylistItemAttributes},
    permission::Capabilities,
};

use librespot_core::{SpotifyUri, date::Date};

use librespot_protocol as protocol;
use protocol::playlist4_external::Item as PlaylistItemMessage;
use protocol::playlist4_external::ListItems as PlaylistItemsMessage;
use protocol::playlist4_external::MetaItem as PlaylistMetaItemMessage;

/// An item in a playlist.
#[derive(Debug, Clone)]
pub struct PlaylistItem {
    /// The Spotify URI of the item.
    pub id: SpotifyUri,
    /// Attributes associated with this item.
    pub attributes: PlaylistItemAttributes,
}

/// A list of [`PlaylistItem`]s.
#[derive(Debug, Clone, Default)]
pub struct PlaylistItems(pub Vec<PlaylistItem>);

impl_deref_wrapped!(PlaylistItems, Vec<PlaylistItem>);

/// A paginated list of playlist items.
#[derive(Debug, Clone)]
pub struct PlaylistItemList {
    /// The current position in the list.
    pub position: i32,
    /// Whether the list is truncated.
    pub is_truncated: bool,
    /// The items in this page.
    pub items: PlaylistItems,
    /// Metadata items associated with the playlist.
    pub meta_items: PlaylistMetaItems,
}

/// Metadata for a playlist item.
#[derive(Debug, Clone)]
pub struct PlaylistMetaItem {
    /// The revision identifier.
    pub revision: SpotifyUri,
    /// The playlist attributes.
    pub attributes: PlaylistAttributes,
    /// The number of items.
    pub length: i32,
    /// The timestamp.
    pub timestamp: Date,
    /// The owner's username.
    pub owner_username: String,
    /// Whether abuse reporting is enabled.
    pub has_abuse_reporting: bool,
    /// The user's capabilities for this playlist.
    pub capabilities: Capabilities,
}

/// A list of [`PlaylistMetaItem`]s.
#[derive(Debug, Clone, Default)]
pub struct PlaylistMetaItems(pub Vec<PlaylistMetaItem>);

impl_deref_wrapped!(PlaylistMetaItems, Vec<PlaylistMetaItem>);

impl TryFrom<&PlaylistItemMessage> for PlaylistItem {
    type Error = librespot_core::Error;
    fn try_from(item: &PlaylistItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: item.try_into()?,
            attributes: item.attributes.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(PlaylistItemMessage, PlaylistItems);

impl TryFrom<&PlaylistItemsMessage> for PlaylistItemList {
    type Error = librespot_core::Error;
    fn try_from(list_items: &PlaylistItemsMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            position: list_items.pos(),
            is_truncated: list_items.truncated(),
            items: list_items.items.as_slice().try_into()?,
            meta_items: list_items.meta_items.as_slice().try_into()?,
        })
    }
}

impl TryFrom<&PlaylistMetaItemMessage> for PlaylistMetaItem {
    type Error = librespot_core::Error;
    fn try_from(item: &PlaylistMetaItemMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            revision: item.try_into()?,
            attributes: item.attributes.get_or_default().try_into()?,
            length: item.length(),
            timestamp: Date::from_timestamp_ms(item.timestamp())?,
            owner_username: item.owner_username().to_owned(),
            has_abuse_reporting: item.abuse_reporting_enabled(),
            capabilities: item.capabilities.get_or_default().into(),
        })
    }
}

impl_try_from_repeated!(PlaylistMetaItemMessage, PlaylistMetaItems);
