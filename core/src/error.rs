//! Unified error type for librespot.
//!
//! The `Error` struct wraps an [`ErrorKind`] (modeled after gRPC status codes) and the
//! original error. Use the constructor methods ([`Error::not_found`],
//! [`Error::unavailable`], etc.) to create typed errors.

use std::{
    error, fmt,
    num::{ParseIntError, TryFromIntError},
    str::Utf8Error,
    string::FromUtf8Error,
};

use base64::DecodeError;
use http::{
    header::{InvalidHeaderName, InvalidHeaderValue, ToStrError},
    method::InvalidMethod,
    status::InvalidStatusCode,
    uri::{InvalidUri, InvalidUriParts},
};
use protobuf::Error as ProtobufError;
use thiserror::Error;
use tokio::sync::{
    AcquireError, TryAcquireError, mpsc::error::SendError, oneshot::error::RecvError,
};
use url::ParseError;

use librespot_oauth::OAuthError;

/// The unified error type for all librespot operations.
///
/// Contains an [`ErrorKind`] category and the underlying error. Construct via
/// the typed helpers: [`Error::not_found`], [`Error::unavailable`], etc.
#[derive(Debug)]
pub struct Error {
    /// The error category.
    pub kind: ErrorKind,
    /// The underlying error.
    pub error: Box<dyn error::Error + Send + Sync>,
}

/// Error category, modeled after gRPC status codes.
///
/// Each variant has a numeric discriminant matching the gRPC specification.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// The operation was cancelled by the caller.
    #[error("The operation was cancelled by the caller")]
    Cancelled = 1,

    /// Unknown error.
    #[error("Unknown error")]
    Unknown = 2,

    /// Client specified an invalid argument.
    #[error("Client specified an invalid argument")]
    InvalidArgument = 3,

    /// Deadline expired before operation could complete.
    #[error("Deadline expired before operation could complete")]
    DeadlineExceeded = 4,

    /// Requested entity was not found.
    #[error("Requested entity was not found")]
    NotFound = 5,

    /// Attempt to create entity that already exists.
    #[error("Attempt to create entity that already exists")]
    AlreadyExists = 6,

    /// Permission denied.
    #[error("Permission denied")]
    PermissionDenied = 7,

    /// No valid authentication credentials.
    #[error("No valid authentication credentials")]
    Unauthenticated = 16,

    /// Resource has been exhausted.
    #[error("Resource has been exhausted")]
    ResourceExhausted = 8,

    /// Invalid state.
    #[error("Invalid state")]
    FailedPrecondition = 9,

    /// Operation aborted.
    #[error("Operation aborted")]
    Aborted = 10,

    /// Operation attempted past the valid range.
    #[error("Operation attempted past the valid range")]
    OutOfRange = 11,

    /// Not implemented.
    #[error("Not implemented")]
    Unimplemented = 12,

    /// Internal error.
    #[error("Internal error")]
    Internal = 13,

    /// Service unavailable.
    #[error("Service unavailable")]
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    #[error("Unrecoverable data loss or corruption")]
    DataLoss = 15,

    /// Operation must not be used.
    #[error("Operation must not be used")]
    DoNotUse = -1,
}

#[derive(Debug, Error)]
struct ErrorMessage(String);

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    /// Creates a new error with the given kind and source error.
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind,
            error: error.into(),
        }
    }

    /// Creates an `Aborted` error.
    pub fn aborted<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Aborted,
            error: error.into(),
        }
    }

    /// Creates an `AlreadyExists` error.
    pub fn already_exists<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::AlreadyExists,
            error: error.into(),
        }
    }

    /// Creates a `Cancelled` error.
    pub fn cancelled<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Cancelled,
            error: error.into(),
        }
    }

    /// Creates a `DataLoss` error.
    pub fn data_loss<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DataLoss,
            error: error.into(),
        }
    }

    /// Creates a `DeadlineExceeded` error.
    pub fn deadline_exceeded<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DeadlineExceeded,
            error: error.into(),
        }
    }

    /// Creates a `DoNotUse` error.
    pub fn do_not_use<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::DoNotUse,
            error: error.into(),
        }
    }

    /// Creates a `FailedPrecondition` error.
    pub fn failed_precondition<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::FailedPrecondition,
            error: error.into(),
        }
    }

    /// Creates an `Internal` error.
    pub fn internal<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Internal,
            error: error.into(),
        }
    }

    /// Creates an `InvalidArgument` error.
    pub fn invalid_argument<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::InvalidArgument,
            error: error.into(),
        }
    }

    /// Creates a `NotFound` error.
    pub fn not_found<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::NotFound,
            error: error.into(),
        }
    }

    /// Creates an `OutOfRange` error.
    pub fn out_of_range<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::OutOfRange,
            error: error.into(),
        }
    }

    /// Creates a `PermissionDenied` error.
    pub fn permission_denied<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::PermissionDenied,
            error: error.into(),
        }
    }

    /// Creates a `ResourceExhausted` error.
    pub fn resource_exhausted<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: error.into(),
        }
    }

    /// Creates an `Unauthenticated` error.
    pub fn unauthenticated<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unauthenticated,
            error: error.into(),
        }
    }

    /// Creates an `Unavailable` error.
    pub fn unavailable<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unavailable,
            error: error.into(),
        }
    }

    /// Creates an `Unimplemented` error.
    pub fn unimplemented<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unimplemented,
            error: error.into(),
        }
    }

    /// Creates an `Unknown` error.
    pub fn unknown<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            kind: ErrorKind::Unknown,
            error: error.into(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{} {{ ", self.kind)?;
        self.error.fmt(fmt)?;
        write!(fmt, " }}")
    }
}

impl From<OAuthError> for Error {
    fn from(err: OAuthError) -> Self {
        use OAuthError::*;
        match err {
            AuthCodeBadUri { .. }
            | AuthCodeNotFound { .. }
            | AuthCodeListenerRead
            | AuthCodeListenerParse => Error::unavailable(err),
            AuthCodeStdinRead
            | AuthCodeListenerBind { .. }
            | AuthCodeListenerTerminated
            | AuthCodeListenerWrite
            | Recv
            | ExchangeCode { .. } => Error::internal(err),
            _ => Error::failed_precondition(err),
        }
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        if err.is::<InvalidHeaderName>()
            || err.is::<InvalidHeaderValue>()
            || err.is::<InvalidMethod>()
            || err.is::<InvalidUri>()
            || err.is::<InvalidUriParts>()
        {
            return Self::new(ErrorKind::InvalidArgument, err);
        }

        if err.is::<InvalidStatusCode>() {
            return Self::new(ErrorKind::FailedPrecondition, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        if err.is_parse() || err.is_parse_status() || err.is_user() {
            return Self::new(ErrorKind::Internal, err);
        }

        if err.is_canceled() {
            return Self::new(ErrorKind::Cancelled, err);
        }

        if err.is_incomplete_message() {
            return Self::new(ErrorKind::DataLoss, err);
        }

        if err.is_body_write_aborted() || err.is_closed() {
            return Self::new(ErrorKind::Aborted, err);
        }

        if err.is_timeout() {
            return Self::new(ErrorKind::DeadlineExceeded, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(err: hyper_util::client::legacy::Error) -> Self {
        if err.is_connect() {
            return Self::new(ErrorKind::Unavailable, err);
        }

        Self::new(ErrorKind::Unknown, err)
    }
}

impl From<time::error::Parse> for Error {
    fn from(err: time::error::Parse) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind as IoErrorKind;
        match err.kind() {
            IoErrorKind::NotFound => Self::new(ErrorKind::NotFound, err),
            IoErrorKind::PermissionDenied => Self::new(ErrorKind::PermissionDenied, err),
            IoErrorKind::AddrInUse | IoErrorKind::AlreadyExists => {
                Self::new(ErrorKind::AlreadyExists, err)
            }
            IoErrorKind::AddrNotAvailable
            | IoErrorKind::ConnectionRefused
            | IoErrorKind::NotConnected => Self::new(ErrorKind::Unavailable, err),
            IoErrorKind::BrokenPipe
            | IoErrorKind::ConnectionReset
            | IoErrorKind::ConnectionAborted => Self::new(ErrorKind::Aborted, err),
            IoErrorKind::Interrupted | IoErrorKind::WouldBlock => {
                Self::new(ErrorKind::Cancelled, err)
            }
            IoErrorKind::InvalidData | IoErrorKind::UnexpectedEof => {
                Self::new(ErrorKind::FailedPrecondition, err)
            }
            IoErrorKind::TimedOut => Self::new(ErrorKind::DeadlineExceeded, err),
            IoErrorKind::InvalidInput => Self::new(ErrorKind::InvalidArgument, err),
            IoErrorKind::WriteZero => Self::new(ErrorKind::ResourceExhausted, err),
            _ => Self::new(ErrorKind::Unknown, err),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Self::new(ErrorKind::InvalidArgument, err)
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Self {
        Self::new(ErrorKind::InvalidArgument, err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Self::new(ErrorKind::Internal, err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Self {
            kind: ErrorKind::Internal,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<AcquireError> for Error {
    fn from(err: AcquireError) -> Self {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<TryAcquireError> for Error {
    fn from(err: TryAcquireError) -> Self {
        Self {
            kind: ErrorKind::ResourceExhausted,
            error: ErrorMessage(err.to_string()).into(),
        }
    }
}

impl From<ToStrError> for Error {
    fn from(err: ToStrError) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Self::new(ErrorKind::FailedPrecondition, err)
    }
}

impl From<protobuf_json_mapping::ParseError> for Error {
    fn from(err: protobuf_json_mapping::ParseError) -> Self {
        Self::failed_precondition(err)
    }
}
