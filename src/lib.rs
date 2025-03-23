//! coalesced supports reading configs from multiple sources
//!
//! ## Usage
//! TODO

pub use coalesced_core::{
    coalesce::{Coalesce, CoalesceExt},
    coalesced::Coalesced,
    priority::{Multiple, Posterior, Prior, Single},
};

pub mod ext {
    pub use coalesced_core::priority::{Access, Length};
}
