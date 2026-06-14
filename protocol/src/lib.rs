// This file is parsed by build.rs
// Each included module will be compiled from the matching .proto definition.
#![allow(missing_docs)]

mod impl_trait;

include!(concat!(env!("OUT_DIR"), "/mod.rs"));
