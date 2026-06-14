//! Audio file and item types.
//!
//! [`AudioItem`] is the unified representation for playable content (tracks
//! and episodes) used by the playback pipeline. It resolves the appropriate
//! files, checks availability, and provides cover art URLs.

/// Audio file format and file mapping types.
pub mod file;
/// Unified audio item representation for playback.
pub mod item;

pub use file::{AudioFileFormat, AudioFiles};
pub use item::{AudioItem, UniqueFields};
