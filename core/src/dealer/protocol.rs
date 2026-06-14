/// Dealer request protocol types.
pub mod request;

pub use request::*;

use std::collections::HashMap;
use std::io::{Error as IoError, Read};

use crate::{Error, deserialize_with::json_proto};
use base64::{DecodeError, Engine, prelude::BASE64_STANDARD};
use flate2::read::GzDecoder;
use log::LevelFilter;
use serde::Deserialize;
use serde_json::Error as SerdeError;
use thiserror::Error;

const IGNORE_UNKNOWN: protobuf_json_mapping::ParseOptions = protobuf_json_mapping::ParseOptions {
    ignore_unknown_fields: true,
    _future_options: (),
};

type JsonValue = serde_json::Value;

#[derive(Debug, Error)]
enum ProtocolError {
    #[error("base64 decoding failed: {0}")]
    Base64(DecodeError),
    #[error("gzip decoding failed: {0}")]
    GZip(IoError),
    #[error("deserialization failed: {0}")]
    Deserialization(SerdeError),
    #[error("payload had more then one value. had {0} values")]
    MoreThenOneValue(usize),
    #[error("received unexpected data {0:#?}")]
    UnexpectedData(PayloadValue),
    #[error("payload was empty")]
    Empty,
}

impl From<ProtocolError> for Error {
    fn from(err: ProtocolError) -> Self {
        match err {
            ProtocolError::UnexpectedData(_) => Error::unavailable(err),
            _ => Error::failed_precondition(err),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct Payload {
    pub compressed: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct WebsocketRequest {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub message_ident: String,
    pub key: String,
    pub payload: Payload,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct WebsocketMessage {
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    #[serde(default)]
    pub payloads: Vec<MessagePayloadValue>,
    pub uri: String,
}

/// A raw payload value from a dealer WebSocket message.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum MessagePayloadValue {
    /// A string payload (typically base64-encoded).
    String(String),
    /// Raw bytes payload.
    Bytes(Vec<u8>),
    /// A JSON payload.
    Json(JsonValue),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum MessageOrRequest {
    Message(WebsocketMessage),
    Request(WebsocketRequest),
}

/// A decoded payload value.
#[derive(Clone, Debug)]
pub enum PayloadValue {
    /// No payload.
    Empty,
    /// Raw bytes.
    Raw(Vec<u8>),
    /// A JSON string.
    Json(String),
}

/// A decoded dealer message with headers, payload, and URI.
#[derive(Clone, Debug)]
pub struct Message {
    /// The message headers.
    pub headers: HashMap<String, String>,
    /// The decoded payload.
    pub payload: PayloadValue,
    /// The message URI.
    pub uri: String,
}

/// A wrapper that tries to deserialize as `T`, falling back to raw JSON.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum FallbackWrapper<T: protobuf::MessageFull> {
    /// Successfully deserialized protobuf message.
    Inner(#[serde(deserialize_with = "json_proto")] T),
    /// Fallback raw JSON value.
    Fallback(JsonValue),
}

impl Message {
    /// Attempts to deserialize the message payload as a protobuf type with JSON fallback.
    pub fn try_from_json<M: protobuf::MessageFull>(
        value: Self,
    ) -> Result<FallbackWrapper<M>, Error> {
        match value.payload {
            PayloadValue::Json(json) => Ok(serde_json::from_str(&json)?),
            other => Err(ProtocolError::UnexpectedData(other).into()),
        }
    }

    /// Deserializes the message payload from raw protobuf bytes.
    pub fn from_raw<M: protobuf::Message>(value: Self) -> Result<M, Error> {
        match value.payload {
            PayloadValue::Raw(bytes) => {
                M::parse_from_bytes(&bytes).map_err(Error::failed_precondition)
            }
            other => Err(ProtocolError::UnexpectedData(other).into()),
        }
    }
}

impl WebsocketMessage {
    pub fn handle_payload(&mut self) -> Result<PayloadValue, Error> {
        if self.payloads.is_empty() {
            return Ok(PayloadValue::Empty);
        } else if self.payloads.len() > 1 {
            return Err(ProtocolError::MoreThenOneValue(self.payloads.len()).into());
        }

        let payload = self.payloads.pop().ok_or(ProtocolError::Empty)?;
        let bytes = match payload {
            MessagePayloadValue::String(string) => BASE64_STANDARD
                .decode(string)
                .map_err(ProtocolError::Base64)?,
            MessagePayloadValue::Bytes(bytes) => bytes,
            MessagePayloadValue::Json(json) => return Ok(PayloadValue::Json(json.to_string())),
        };

        handle_transfer_encoding(&self.headers, bytes).map(PayloadValue::Raw)
    }
}

impl WebsocketRequest {
    pub fn handle_payload(&self) -> Result<Request, Error> {
        let payload_bytes = BASE64_STANDARD
            .decode(&self.payload.compressed)
            .map_err(ProtocolError::Base64)?;

        let payload = handle_transfer_encoding(&self.headers, payload_bytes)?;
        let payload = String::from_utf8(payload)?;

        if log::max_level() >= LevelFilter::Trace {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload) {
                trace!("websocket request: {json:#?}");
            } else {
                trace!("websocket request: {payload}");
            }
        }

        serde_json::from_str(&payload)
            .map_err(ProtocolError::Deserialization)
            .map_err(Into::into)
    }
}

fn handle_transfer_encoding(
    headers: &HashMap<String, String>,
    data: Vec<u8>,
) -> Result<Vec<u8>, Error> {
    let encoding = headers.get("Transfer-Encoding").map(String::as_str);
    if let Some(encoding) = encoding {
        trace!("message was sent with {encoding} encoding ");
    } else {
        trace!("message was sent with no encoding ");
    }

    if !matches!(encoding, Some("gzip")) {
        return Ok(data);
    }

    let mut gz = GzDecoder::new(&data[..]);
    let mut bytes = vec![];
    match gz.read_to_end(&mut bytes) {
        Ok(i) if i == bytes.len() => Ok(bytes),
        Ok(_) => Err(Error::failed_precondition(
            "read bytes mismatched with expected bytes",
        )),
        Err(why) => Err(ProtocolError::GZip(why).into()),
    }
}
