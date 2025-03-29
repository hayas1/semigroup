//! coalesced supports reading configs from multiple sources
//!
//! ## Usage
//! TODO

pub use coalesced_base::{
    coalesce::Coalesce,
    coalesced::{Coalesced, History, IntoHistory},
    extension::{Extension, WithExt},
};

#[cfg(feature = "derive")]
pub use coalesced_derive::Coalesce;
