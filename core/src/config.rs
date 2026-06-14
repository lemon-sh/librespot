//! Configuration types for [`Session`](crate::Session).

use std::{fmt, path::PathBuf, str::FromStr};

use librespot_protocol::devices::DeviceType as ProtoDeviceType;
use url::Url;

pub(crate) const KEYMASTER_CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
pub(crate) const ANDROID_CLIENT_ID: &str = "9a8d2f0ce77a4e248bb71fefcb557637";
pub(crate) const IOS_CLIENT_ID: &str = "58bd3c95768941ea9eb4350aaa033eb3";

// Easily adjust the current platform to mock the behavior on it. If for example
// android or ios needs to be mocked, the `os_version` has to be set to a valid version.
// Otherwise, client-token or login5 requests will fail with a generic invalid-credential error.
/// See [std::env::consts::OS]
pub const OS: &str = std::env::consts::OS;

// valid versions for some os:
// 'android': 30
// 'ios': 17
/// See [sysinfo::System::os_version]
pub fn os_version() -> String {
    sysinfo::System::os_version().unwrap_or("0".into())
}

/// Configuration for a [`Session`](crate::Session).
///
/// Use [`SessionConfig::default`] to get sensible defaults, or customize individual fields.
#[derive(Clone, Debug)]
pub struct SessionConfig {
    /// OAuth client ID. Defaults to the keymaster client ID for desktop.
    pub client_id: String,
    /// Unique device identifier. Defaults to a random UUID.
    pub device_id: String,
    /// Optional HTTP proxy URL.
    pub proxy: Option<Url>,
    /// Optional port override for access point connections.
    pub ap_port: Option<u16>,
    /// Temporary directory for cached data.
    pub tmp_dir: PathBuf,
    /// Override autoplay setting. `None` uses the server-side user attribute.
    pub autoplay: Option<bool>,
}

impl SessionConfig {
    pub(crate) fn default_for_os(os: &str) -> Self {
        let device_id = uuid::Uuid::new_v4().as_hyphenated().to_string();
        let client_id = match os {
            "android" => ANDROID_CLIENT_ID,
            "ios" => IOS_CLIENT_ID,
            _ => KEYMASTER_CLIENT_ID,
        }
        .to_owned();

        Self {
            client_id,
            device_id,
            proxy: None,
            ap_port: None,
            tmp_dir: std::env::temp_dir(),
            autoplay: None,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::default_for_os(OS)
    }
}

/// The type of device this instance represents in Spotify Connect.
///
/// Affects the icon shown in the Spotify client's device picker.
#[derive(Clone, Copy, Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub enum DeviceType {
    /// Unknown device type.
    Unknown = 0,
    /// A desktop or laptop computer.
    Computer = 1,
    /// A tablet device.
    Tablet = 2,
    /// A smartphone.
    Smartphone = 3,
    /// A speaker (default).
    #[default]
    Speaker = 4,
    /// A smart TV.
    Tv = 5,
    /// An audio/video receiver.
    Avr = 6,
    /// A set-top box.
    Stb = 7,
    /// An audio dongle.
    AudioDongle = 8,
    /// A game console.
    GameConsole = 9,
    /// A Chromecast Audio device.
    CastAudio = 10,
    /// A Chromecast Video device.
    CastVideo = 11,
    /// An automobile infotainment system.
    Automobile = 12,
    /// A smartwatch.
    Smartwatch = 13,
    /// A Chromebook.
    Chromebook = 14,
    /// An unknown Spotify device type.
    UnknownSpotify = 100,
    /// A Spotify Car Thing.
    CarThing = 101,
    /// An observer device.
    Observer = 102,
}

impl FromStr for DeviceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::DeviceType::*;
        match s.to_lowercase().as_ref() {
            "computer" => Ok(Computer),
            "tablet" => Ok(Tablet),
            "smartphone" => Ok(Smartphone),
            "speaker" => Ok(Speaker),
            "tv" => Ok(Tv),
            "avr" => Ok(Avr),
            "stb" => Ok(Stb),
            "audiodongle" => Ok(AudioDongle),
            "gameconsole" => Ok(GameConsole),
            "castaudio" => Ok(CastAudio),
            "castvideo" => Ok(CastVideo),
            "automobile" => Ok(Automobile),
            "smartwatch" => Ok(Smartwatch),
            "chromebook" => Ok(Chromebook),
            "carthing" => Ok(CarThing),
            _ => Err(()),
        }
    }
}

impl From<&DeviceType> for &str {
    fn from(d: &DeviceType) -> &'static str {
        use self::DeviceType::*;
        match d {
            Unknown => "Unknown",
            Computer => "Computer",
            Tablet => "Tablet",
            Smartphone => "Smartphone",
            Speaker => "Speaker",
            Tv => "TV",
            Avr => "AVR",
            Stb => "STB",
            AudioDongle => "AudioDongle",
            GameConsole => "GameConsole",
            CastAudio => "CastAudio",
            CastVideo => "CastVideo",
            Automobile => "Automobile",
            Smartwatch => "Smartwatch",
            Chromebook => "Chromebook",
            UnknownSpotify => "UnknownSpotify",
            CarThing => "CarThing",
            Observer => "Observer",
        }
    }
}

impl From<DeviceType> for &str {
    fn from(d: DeviceType) -> &'static str {
        (&d).into()
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.into();
        f.write_str(str)
    }
}

impl From<DeviceType> for ProtoDeviceType {
    fn from(value: DeviceType) -> Self {
        match value {
            DeviceType::Unknown => ProtoDeviceType::UNKNOWN,
            DeviceType::Computer => ProtoDeviceType::COMPUTER,
            DeviceType::Tablet => ProtoDeviceType::TABLET,
            DeviceType::Smartphone => ProtoDeviceType::SMARTPHONE,
            DeviceType::Speaker => ProtoDeviceType::SPEAKER,
            DeviceType::Tv => ProtoDeviceType::TV,
            DeviceType::Avr => ProtoDeviceType::AVR,
            DeviceType::Stb => ProtoDeviceType::STB,
            DeviceType::AudioDongle => ProtoDeviceType::AUDIO_DONGLE,
            DeviceType::GameConsole => ProtoDeviceType::GAME_CONSOLE,
            DeviceType::CastAudio => ProtoDeviceType::CAST_VIDEO,
            DeviceType::CastVideo => ProtoDeviceType::CAST_AUDIO,
            DeviceType::Automobile => ProtoDeviceType::AUTOMOBILE,
            DeviceType::Smartwatch => ProtoDeviceType::SMARTWATCH,
            DeviceType::Chromebook => ProtoDeviceType::CHROMEBOOK,
            DeviceType::UnknownSpotify => ProtoDeviceType::UNKNOWN_SPOTIFY,
            DeviceType::CarThing => ProtoDeviceType::CAR_THING,
            DeviceType::Observer => ProtoDeviceType::OBSERVER,
        }
    }
}
