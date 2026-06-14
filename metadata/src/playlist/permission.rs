use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated_copy};

use librespot_protocol as protocol;
use protocol::playlist_permission::Capabilities as CapabilitiesMessage;
use protocol::playlist_permission::PermissionLevel;

/// A user's permission capabilities for a playlist.
#[derive(Debug, Clone)]
pub struct Capabilities {
    /// Whether the user can view the playlist.
    pub can_view: bool,
    /// Whether the user can manage permissions for the playlist.
    pub can_administrate_permissions: bool,
    /// The permission levels that can be granted.
    pub grantable_levels: PermissionLevels,
    /// Whether the user can edit playlist metadata.
    pub can_edit_metadata: bool,
    /// Whether the user can edit playlist items.
    pub can_edit_items: bool,
    /// Whether the user can cancel their own membership.
    pub can_cancel_membership: bool,
}

/// A list of [`PermissionLevel`]s.
#[derive(Debug, Clone, Default)]
pub struct PermissionLevels(pub Vec<PermissionLevel>);

impl_deref_wrapped!(PermissionLevels, Vec<PermissionLevel>);

impl From<&CapabilitiesMessage> for Capabilities {
    fn from(playlist: &CapabilitiesMessage) -> Self {
        Self {
            can_view: playlist.can_view(),
            can_administrate_permissions: playlist.can_administrate_permissions(),
            grantable_levels: PermissionLevels(
                playlist
                    .grantable_level
                    .iter()
                    .map(protobuf::EnumOrUnknown::enum_value_or_default)
                    .collect(),
            ),
            can_edit_metadata: playlist.can_edit_metadata(),
            can_edit_items: playlist.can_edit_items(),
            can_cancel_membership: playlist.can_cancel_membership(),
        }
    }
}

impl_from_repeated_copy!(PermissionLevel, PermissionLevels);
