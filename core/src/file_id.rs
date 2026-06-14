//! Audio and image file identifiers.
//!
//! A [`FileId`] is a 20-byte SHA1 hash that uniquely identifies an audio file
//! (or image) on Spotify's CDN. It is used by `AudioFile` to locate and fetch
//! encrypted audio data.

use std::fmt::{self, Write};

use librespot_protocol as protocol;

const RAW_LEN: usize = 20;

/// A 20-byte identifier for an audio or image file on Spotify's CDN.
///
/// Encoded as 40-character hex strings. Used by the audio fetching pipeline
/// to locate encrypted audio chunks.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; RAW_LEN]);

impl FileId {
    /// Creates a `FileId` from a raw byte slice.
    pub fn from_raw(src: &[u8]) -> FileId {
        let mut dst = [0u8; RAW_LEN];
        let len = src.len();
        // some tracks return 16 instead of 20 bytes: #1188
        if len <= RAW_LEN {
            dst[..len].clone_from_slice(src);
        }
        FileId(dst)
    }

    /// Returns the `FileId` as a 40-character lowercase hex string.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_base16(&self) -> String {
        let mut s = String::new();
        for b in &self.0 {
            write!(&mut s, "{b:02x}").unwrap();
        }
        s
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}

impl From<&[u8]> for FileId {
    fn from(src: &[u8]) -> Self {
        Self::from_raw(src)
    }
}
impl From<&protocol::metadata::Image> for FileId {
    fn from(image: &protocol::metadata::Image) -> Self {
        Self::from(image.file_id())
    }
}

impl From<&protocol::metadata::AudioFile> for FileId {
    fn from(file: &protocol::metadata::AudioFile) -> Self {
        Self::from(file.file_id())
    }
}

impl From<&protocol::metadata::VideoFile> for FileId {
    fn from(video: &protocol::metadata::VideoFile) -> Self {
        Self::from(video.file_id())
    }
}
