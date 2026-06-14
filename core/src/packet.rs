//! Spotify packet type definitions.
//!
//! Defines the [`PacketType`] enum for all packet types sent over the
//! Shannon-encrypted connection.

// Ported from librespot-java. Relicensed under MIT with permission.

use num_derive::{FromPrimitive, ToPrimitive};

/// Spotify packet type definitions.
///
/// Each variant maps to a specific command or event code used in
/// the Shannon-encrypted protocol.
#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum PacketType {
    /// A secret block from the server.
    SecretBlock = 0x02,
    /// Server keep-alive ping.
    Ping = 0x04,
    /// Audio data chunk.
    StreamChunk = 0x08,
    /// Audio data chunk response/acknowledgment.
    StreamChunkRes = 0x09,
    /// Error on a data channel.
    ChannelError = 0x0a,
    /// Channel aborted by the server.
    ChannelAbort = 0x0b,
    /// Request for an audio decryption key.
    RequestKey = 0x0c,
    /// AES key response for audio decryption.
    AesKey = 0x0d,
    /// AES key error response.
    AesKeyError = 0x0e,
    /// Image data.
    Image = 0x19,
    /// Country code of the user.
    CountryCode = 0x1b,
    /// Server keep-alive pong (unused, see PongAck).
    Pong = 0x49,
    /// Acknowledgment of a client pong.
    PongAck = 0x4a,
    /// Pause playback command.
    Pause = 0x4b,
    /// Product information (premium status, etc.).
    ProductInfo = 0x50,
    /// Legacy welcome message from the server.
    LegacyWelcome = 0x69,
    /// License version information.
    LicenseVersion = 0x76,
    /// Client login request.
    Login = 0xab,
    /// Access point welcome after successful authentication.
    APWelcome = 0xac,
    /// Authentication failure response.
    AuthFailure = 0xad,
    /// Mercury request packet.
    MercuryReq = 0xb2,
    /// Mercury subscription packet.
    MercurySub = 0xb3,
    /// Mercury unsubscription packet.
    MercuryUnsub = 0xb4,
    /// Mercury event (push notification) packet.
    MercuryEvent = 0xb5,
    /// Track playback ended time tracking.
    TrackEndedTime = 0x82,
    /// Unknown packet with all-zero payload.
    UnknownDataAllZeros = 0x1f,
    /// Preferred locale setting from the user.
    PreferredLocale = 0x74,
    /// Unknown packet type (0x0f).
    Unknown0x0f = 0x0f,
    /// Unknown packet type (0x10).
    Unknown0x10 = 0x10,
    /// Unknown packet type (0x4f).
    Unknown0x4f = 0x4f,

    // TODO - occurs when subscribing with an empty URI. Maybe a MercuryError?
    // Payload: b"\0\x08\0\0\0\0\0\0\0\0\x01\0\x01\0\x03 \xb0\x06"
    /// Unknown packet type (0xb6), possibly related to Mercury subscription errors.
    Unknown0xb6 = 0xb6,
}
