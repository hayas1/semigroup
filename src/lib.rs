//! TODO doc

pub use coalesced_core::{
    coalesce::{Coalesce, CoalesceExt},
    coalesced::Coalesced,
    priority::{Multiple, Posterior, Prior, Single},
};

pub mod ext {
    pub use coalesced_core::priority::{Access, Length};
}
