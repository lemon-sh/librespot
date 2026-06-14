use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use librespot_core::FileId;

use crate::util::impl_deref_wrapped;
use librespot_protocol as protocol;
use protocol::metadata::AudioFile as AudioFileMessage;

use librespot_protocol::metadata::audio_file::Format;
use protobuf::Enum;

/// Supported audio file formats with their bitrates.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AudioFileFormat {
    /// OGG Vorbis at 96 kbps.
    OGG_VORBIS_96,
    /// OGG Vorbis at 160 kbps.
    OGG_VORBIS_160,
    /// OGG Vorbis at 320 kbps.
    OGG_VORBIS_320,
    /// MP3 at 256 kbps.
    MP3_256,
    /// MP3 at 320 kbps.
    MP3_320,
    /// MP3 at 160 kbps.
    MP3_160,
    /// MP3 at 96 kbps.
    MP3_96,
    /// MP3 at 160 kbps (encrypted).
    MP3_160_ENC,
    /// AAC at 24 kbps.
    AAC_24,
    /// AAC at 48 kbps.
    AAC_48,
    /// FLAC lossless.
    FLAC_FLAC,
    /// xHE-AAC at 24 kbps.
    XHE_AAC_24,
    /// xHE-AAC at 16 kbps.
    XHE_AAC_16,
    /// xHE-AAC at 12 kbps.
    XHE_AAC_12,
    /// FLAC lossless at 24-bit.
    FLAC_FLAC_24BIT,
    /// AAC at 160 kbps (not defined in protobuf).
    AAC_160,
    /// AAC at 320 kbps (not defined in protobuf).
    AAC_320,
    /// MP4 at 128 kbps (not defined in protobuf).
    MP4_128,
    /// Unknown format (not defined in protobuf).
    OTHER5,
}

impl TryFrom<i32> for AudioFileFormat {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            10 => AudioFileFormat::AAC_160,
            11 => AudioFileFormat::AAC_320,
            12 => AudioFileFormat::MP4_128,
            13 => AudioFileFormat::OTHER5,
            _ => Format::from_i32(value).ok_or(value)?.into(),
        })
    }
}

impl From<Format> for AudioFileFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::OGG_VORBIS_96 => AudioFileFormat::OGG_VORBIS_96,
            Format::OGG_VORBIS_160 => AudioFileFormat::OGG_VORBIS_160,
            Format::OGG_VORBIS_320 => AudioFileFormat::OGG_VORBIS_320,
            Format::MP3_256 => AudioFileFormat::MP3_256,
            Format::MP3_320 => AudioFileFormat::MP3_320,
            Format::MP3_160 => AudioFileFormat::MP3_160,
            Format::MP3_96 => AudioFileFormat::MP3_96,
            Format::MP3_160_ENC => AudioFileFormat::MP3_160_ENC,
            Format::AAC_24 => AudioFileFormat::AAC_24,
            Format::AAC_48 => AudioFileFormat::AAC_48,
            Format::FLAC_FLAC => AudioFileFormat::FLAC_FLAC,
            Format::XHE_AAC_24 => AudioFileFormat::XHE_AAC_24,
            Format::XHE_AAC_16 => AudioFileFormat::XHE_AAC_16,
            Format::XHE_AAC_12 => AudioFileFormat::XHE_AAC_12,
            Format::FLAC_FLAC_24BIT => AudioFileFormat::FLAC_FLAC_24BIT,
        }
    }
}

/// A mapping from audio file formats to their file identifiers.
#[derive(Debug, Clone, Default)]
pub struct AudioFiles(pub HashMap<AudioFileFormat, FileId>);

impl_deref_wrapped!(AudioFiles, HashMap<AudioFileFormat, FileId>);

impl AudioFiles {
    /// Returns `true` if the format is OGG Vorbis.
    pub fn is_ogg_vorbis(format: AudioFileFormat) -> bool {
        matches!(
            format,
            AudioFileFormat::OGG_VORBIS_320
                | AudioFileFormat::OGG_VORBIS_160
                | AudioFileFormat::OGG_VORBIS_96
        )
    }

    /// Returns `true` if the format is MP3.
    pub fn is_mp3(format: AudioFileFormat) -> bool {
        matches!(
            format,
            AudioFileFormat::MP3_320
                | AudioFileFormat::MP3_256
                | AudioFileFormat::MP3_160
                | AudioFileFormat::MP3_96
                | AudioFileFormat::MP3_160_ENC
        )
    }

    /// Returns `true` if the format is FLAC.
    pub fn is_flac(format: AudioFileFormat) -> bool {
        matches!(format, AudioFileFormat::FLAC_FLAC)
    }

    /// Returns the MIME type string for the given format, if known.
    pub fn mime_type(format: AudioFileFormat) -> Option<&'static str> {
        if Self::is_ogg_vorbis(format) {
            Some("audio/ogg")
        } else if Self::is_mp3(format) {
            Some("audio/mpeg")
        } else if Self::is_flac(format) {
            Some("audio/flac")
        } else {
            None
        }
    }
}

impl From<&[AudioFileMessage]> for AudioFiles {
    fn from(files: &[AudioFileMessage]) -> Self {
        let audio_files: HashMap<AudioFileFormat, FileId> = files
            .iter()
            .filter_map(|file| {
                let file_id = FileId::from(file.file_id());
                let format = file
                    .format
                    .ok_or(format!("Ignoring file <{file_id}> with unspecified format",))
                    .and_then(|format| match format.enum_value() {
                        Ok(f) => Ok((f.into(), file_id)),
                        Err(unknown) => Err(format!(
                            "Ignoring file <{file_id}> with unknown format {unknown}",
                        )),
                    });

                if let Err(ref why) = format {
                    trace!("{why}");
                }

                format.ok()
            })
            .collect();

        AudioFiles(audio_files)
    }
}
