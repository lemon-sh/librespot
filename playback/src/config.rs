//! Playback configuration types.
//!
//! Configures bitrate, audio format, normalisation, and volume control.
use std::{mem, path::PathBuf, str::FromStr, time::Duration};

pub use crate::dither::{DithererBuilder, TriangularDitherer, mk_ditherer};
use crate::{convert::i24, player::duration_to_coefficient};

/// Audio bitrate setting for playback.
///
/// Higher bitrates use more bandwidth but provide better audio quality.
#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum Bitrate {
    /// 96 kbps. Lowest quality, suitable for very constrained connections.
    Bitrate96,
    /// 160 kbps. Default balance between quality and bandwidth.
    #[default]
    Bitrate160,
    /// 320 kbps. Highest quality, requires more bandwidth.
    Bitrate320,
}

impl FromStr for Bitrate {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "96" => Ok(Self::Bitrate96),
            "160" => Ok(Self::Bitrate160),
            "320" => Ok(Self::Bitrate320),
            _ => Err(()),
        }
    }
}

/// Output audio format for sample conversion.
///
/// Determines the PCM format sent to the audio backend.
#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum AudioFormat {
    /// 64-bit floating point.
    F64,
    /// 32-bit floating point.
    F32,
    /// 32-bit signed integer (24-bit audio padded to 32 bits).
    S32,
    /// 24-bit signed integer packed in 32-bit word.
    S24,
    /// 24-bit signed integer in a 3-byte array.
    S24_3,
    /// 16-bit signed integer. Default format.
    #[default]
    S16,
}

impl FromStr for AudioFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "F64" => Ok(Self::F64),
            "F32" => Ok(Self::F32),
            "S32" => Ok(Self::S32),
            "S24" => Ok(Self::S24),
            "S24_3" => Ok(Self::S24_3),
            "S16" => Ok(Self::S16),
            _ => Err(()),
        }
    }
}

impl AudioFormat {
    /// Returns the size in bytes of a single sample in this format.
    // not used by all backends
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        match self {
            Self::F64 => mem::size_of::<f64>(),
            Self::F32 => mem::size_of::<f32>(),
            Self::S24_3 => mem::size_of::<i24>(),
            Self::S16 => mem::size_of::<i16>(),
            _ => mem::size_of::<i32>(), // S32 and S24 are both stored in i32
        }
    }
}

/// Type of volume normalisation to apply.
///
/// Determines whether to use track-level or album-level gain values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NormalisationType {
    /// Use album-level gain values. Provides consistent loudness across an album.
    Album,
    /// Use track-level gain values. Normalises each track independently.
    Track,
    /// Automatically choose between track and album based on context.
    #[default]
    Auto,
}

impl FromStr for NormalisationType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "album" => Ok(Self::Album),
            "track" => Ok(Self::Track),
            "auto" => Ok(Self::Auto),
            _ => Err(()),
        }
    }
}

/// Method used for volume normalisation.
///
/// Controls how gain is applied to audio samples.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum NormalisationMethod {
    /// Simple gain multiplication with peak limiting to prevent clipping.
    Basic,
    /// Dynamic range compression with soft-knee limiter for smoother results.
    #[default]
    Dynamic,
}

impl FromStr for NormalisationMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "basic" => Ok(Self::Basic),
            "dynamic" => Ok(Self::Dynamic),
            _ => Err(()),
        }
    }
}

/// Configuration for the [`Player`](crate::player::Player).
#[derive(Clone)]
pub struct PlayerConfig {
    /// Audio bitrate (96, 160, or 320 kbps).
    pub bitrate: Bitrate,
    /// Enable gapless playback between tracks.
    pub gapless: bool,
    /// Enable raw Ogg passthrough (no decoding).
    pub passthrough: bool,

    /// Enable volume normalisation.
    pub normalisation: bool,
    /// Normalisation type (track, album, or auto).
    pub normalisation_type: NormalisationType,
    /// Normalisation method (basic or dynamic).
    pub normalisation_method: NormalisationMethod,
    /// Pre-gain in dB applied before normalisation.
    pub normalisation_pregain_db: f64,
    /// Threshold in dBFS above which normalisation kicks in.
    pub normalisation_threshold_dbfs: f64,
    /// Attack time constant for the normalisation gain.
    pub normalisation_attack_cf: f64,
    /// Release time constant for the normalisation gain.
    pub normalisation_release_cf: f64,
    /// Knee width in dB for the normalisation compressor.
    pub normalisation_knee_db: f64,

    /// Directories to search for local files.
    pub local_file_directories: Vec<PathBuf>,

    /// Optional ditherer builder. Default: triangular dithering.
    pub ditherer: Option<DithererBuilder>,
    /// Optional interval for periodic position update events.
    pub position_update_interval: Option<Duration>,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            bitrate: Bitrate::default(),
            gapless: true,
            normalisation: false,
            normalisation_type: NormalisationType::default(),
            normalisation_method: NormalisationMethod::default(),
            normalisation_pregain_db: 0.0,
            normalisation_threshold_dbfs: -2.0,
            normalisation_attack_cf: duration_to_coefficient(Duration::from_millis(5)),
            normalisation_release_cf: duration_to_coefficient(Duration::from_millis(100)),
            normalisation_knee_db: 5.0,
            passthrough: false,
            ditherer: Some(mk_ditherer::<TriangularDitherer>),
            position_update_interval: None,
            local_file_directories: Vec::new(),
        }
    }
}

/// Volume control curve type.
///
/// Controls how linear volume values map to perceived loudness.
/// Fields specify the dB range for the volume control curve.
#[derive(Clone, Copy, Debug)]
pub enum VolumeCtrl {
    /// Cubic mapping with the given dB range. Mimics ALSA's native mixer curve.
    Cubic(f64),
    /// Fixed volume (no attenuation).
    Fixed,
    /// Linear mapping (direct proportionality).
    Linear,
    /// Logarithmic mapping with the given dB range. Best perceptual linearity.
    Log(f64),
}

impl FromStr for VolumeCtrl {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_range(s, Self::DEFAULT_DB_RANGE)
    }
}

impl Default for VolumeCtrl {
    fn default() -> VolumeCtrl {
        VolumeCtrl::Log(Self::DEFAULT_DB_RANGE)
    }
}

impl VolumeCtrl {
    /// Maximum volume value (u16::MAX = 65535).
    pub const MAX_VOLUME: u16 = u16::MAX;

    /// Default dB range for logarithmic and cubic volume curves (60 dB).
    // Taken from: https://www.dr-lex.be/info-stuff/volumecontrols.html
    pub const DEFAULT_DB_RANGE: f64 = 60.0;

    /// Parses a volume control type from a string with a custom dB range.
    pub fn from_str_with_range(s: &str, db_range: f64) -> Result<Self, <Self as FromStr>::Err> {
        use self::VolumeCtrl::*;
        match s.to_lowercase().as_ref() {
            "cubic" => Ok(Cubic(db_range)),
            "fixed" => Ok(Fixed),
            "linear" => Ok(Linear),
            "log" => Ok(Log(db_range)),
            _ => Err(()),
        }
    }
}
