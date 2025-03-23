//! coalesced supports reading configs from multiple sources
//!
//! ## Usage
//! TODO

pub use coalesced_base::{
    coalesced::{Coalesced, History, IntoHistory},
    Coalesce,
};

#[cfg(feature = "derive")]
pub use coalesced_derive::Coalesce;
