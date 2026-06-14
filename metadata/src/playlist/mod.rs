/// Playlist annotation metadata (description, pictures, abuse reporting).
pub mod annotation;
/// Playlist attribute types (format attributes, item attributes, partial attributes).
pub mod attribute;
/// Playlist diff information (revisions and operations).
pub mod diff;
/// Playlist item types (items, meta items, item lists).
pub mod item;
/// Playlist list metadata and the main [`Playlist`](list::Playlist) type.
pub mod list;
/// Playlist operation types (add, remove, move, update attributes).
pub mod operation;
/// Playlist permission types (capabilities, permission levels).
pub mod permission;

pub use annotation::PlaylistAnnotation;
pub use list::Playlist;
