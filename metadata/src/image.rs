use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated, impl_try_from_repeated};

use librespot_core::{FileId, SpotifyUri};

use librespot_protocol as protocol;
use protocol::metadata::Image as ImageMessage;
use protocol::metadata::ImageGroup;
pub use protocol::metadata::image::Size as ImageSize;
use protocol::playlist_annotate3::TranscodedPicture as TranscodedPictureMessage;
use protocol::playlist4_external::PictureSize as PictureSizeMessage;

/// An image associated with a Spotify resource (e.g., cover art).
#[derive(Debug, Clone)]
pub struct Image {
    /// The file identifier for the image data.
    pub id: FileId,
    /// The size category of the image.
    pub size: ImageSize,
    /// The width of the image in pixels.
    pub width: i32,
    /// The height of the image in pixels.
    pub height: i32,
}

/// A list of [`Image`]s.
#[derive(Debug, Clone, Default)]
pub struct Images(pub Vec<Image>);

impl From<&ImageGroup> for Images {
    fn from(image_group: &ImageGroup) -> Self {
        Self(image_group.image.iter().map(Into::into).collect())
    }
}

impl_deref_wrapped!(Images, Vec<Image>);

/// A named picture size variant with a URL.
#[derive(Debug, Clone)]
pub struct PictureSize {
    /// The name of the target size (e.g., `"Standard"`, `"Large"`).
    pub target_name: String,
    /// The URL to the image.
    pub url: String,
}

/// A list of [`PictureSize`]s.
#[derive(Debug, Clone, Default)]
pub struct PictureSizes(pub Vec<PictureSize>);

impl_deref_wrapped!(PictureSizes, Vec<PictureSize>);

/// A transcoded picture with a target name and Spotify URI.
#[derive(Debug, Clone)]
pub struct TranscodedPicture {
    /// The name of the transcoded target.
    pub target_name: String,
    /// The Spotify URI of the transcoded picture.
    pub uri: SpotifyUri,
}

/// A list of [`TranscodedPicture`]s.
#[derive(Debug, Clone)]
pub struct TranscodedPictures(pub Vec<TranscodedPicture>);

impl_deref_wrapped!(TranscodedPictures, Vec<TranscodedPicture>);

impl From<&ImageMessage> for Image {
    fn from(image: &ImageMessage) -> Self {
        Self {
            id: image.into(),
            size: image.size(),
            width: image.width(),
            height: image.height(),
        }
    }
}

impl_from_repeated!(ImageMessage, Images);

impl From<&PictureSizeMessage> for PictureSize {
    fn from(size: &PictureSizeMessage) -> Self {
        Self {
            target_name: size.target_name().to_owned(),
            url: size.url().to_owned(),
        }
    }
}

impl_from_repeated!(PictureSizeMessage, PictureSizes);

impl TryFrom<&TranscodedPictureMessage> for TranscodedPicture {
    type Error = librespot_core::Error;
    fn try_from(picture: &TranscodedPictureMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            target_name: picture.target_name().to_owned(),
            uri: picture.try_into()?,
        })
    }
}

impl_try_from_repeated!(TranscodedPictureMessage, TranscodedPictures);
