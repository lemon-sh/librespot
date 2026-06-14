use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use thiserror::Error;

use crate::util::{impl_deref_wrapped, impl_try_from_repeated};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::metadata::Availability as AvailabilityMessage;

/// Result type indicating whether an audio item is available for playback.
pub type AudioItemAvailability = Result<(), UnavailabilityReason>;

/// Availability information for a Spotify resource in a particular catalogue.
#[derive(Debug, Clone)]
pub struct Availability {
    /// Catalogue strings this availability applies to.
    pub catalogue_strs: Vec<String>,
    /// The date from which the resource becomes available.
    pub start: Date,
}

/// A list of [`Availability`] entries.
#[derive(Debug, Clone, Default)]
pub struct Availabilities(pub Vec<Availability>);

impl_deref_wrapped!(Availabilities, Vec<Availability>);

/// Reasons why an audio item may not be available for playback.
#[derive(Debug, Copy, Clone, Error)]
pub enum UnavailabilityReason {
    /// The user's country is on a blacklist for this resource.
    #[error("blacklist present and country on it")]
    Blacklisted,
    /// The resource is not yet available (embargo date is in the future).
    #[error("available date is in the future")]
    Embargo,
    /// Required availability data was not present.
    #[error("required data was not present")]
    NoData,
    /// A whitelist exists and the user's country is not on it.
    #[error("whitelist present and country not on it")]
    NotWhitelisted,
}

impl TryFrom<&AvailabilityMessage> for Availability {
    type Error = librespot_core::Error;
    fn try_from(availability: &AvailabilityMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            catalogue_strs: availability.catalogue_str.to_vec(),
            start: availability.start.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(AvailabilityMessage, Availabilities);
