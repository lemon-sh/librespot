//! Audio file and item types.
//!
//! [`AudioItem`] is the unified representation for playable content (tracks
//! and episodes) used by the playback pipeline. It resolves the appropriate
//! files, checks availability, and provides cover art URLs.

pub mod file;
pub mod item;

pub use file::{AudioFileFormat, AudioFiles};
pub use item::{AudioItem, UniqueFields};
