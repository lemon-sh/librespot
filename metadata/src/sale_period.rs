use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    restriction::Restrictions,
    util::{impl_deref_wrapped, impl_try_from_repeated},
};

use librespot_core::date::Date;

use librespot_protocol as protocol;
use protocol::metadata::SalePeriod as SalePeriodMessage;

/// A time period during which a Spotify resource is available for sale,
/// along with any applicable restrictions.
#[derive(Debug, Clone)]
pub struct SalePeriod {
    /// Restrictions that apply during this sale period.
    pub restrictions: Restrictions,
    /// The start date of the sale period.
    pub start: Date,
    /// The end date of the sale period.
    pub end: Date,
}

/// A list of [`SalePeriod`]s.
#[derive(Debug, Clone, Default)]
pub struct SalePeriods(pub Vec<SalePeriod>);

impl_deref_wrapped!(SalePeriods, Vec<SalePeriod>);

impl TryFrom<&SalePeriodMessage> for SalePeriod {
    type Error = librespot_core::Error;
    fn try_from(sale_period: &SalePeriodMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            restrictions: sale_period.restriction.as_slice().into(),
            start: sale_period.start.get_or_default().try_into()?,
            end: sale_period.end.get_or_default().try_into()?,
        })
    }
}

impl_try_from_repeated!(SalePeriodMessage, SalePeriods);
