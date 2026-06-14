use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::util::{impl_deref_wrapped, impl_from_repeated};

use librespot_protocol as protocol;
use protocol::metadata::Copyright as CopyrightMessage;
pub use protocol::metadata::copyright::Type as CopyrightType;

/// A copyright notice for a Spotify resource.
#[derive(Debug, Clone)]
pub struct Copyright {
    /// The type of copyright (e.g., performance, sound recording).
    pub copyright_type: CopyrightType,
    /// The copyright text.
    pub text: String,
}

/// A list of [`Copyright`]s.
#[derive(Debug, Clone, Default)]
pub struct Copyrights(pub Vec<Copyright>);

impl_deref_wrapped!(Copyrights, Vec<Copyright>);

impl From<&CopyrightMessage> for Copyright {
    fn from(copyright: &CopyrightMessage) -> Self {
        Self {
            copyright_type: copyright.type_(),
            text: copyright.text().to_owned(),
        }
    }
}

impl_from_repeated!(CopyrightMessage, Copyrights);
