mod coalesce;
mod coalesced;
mod extension;
mod priority;

#[cfg(feature = "clap")]
mod clap;
#[cfg(feature = "serde")]
mod serde;

pub use coalesce::{Coalesce, CoalesceExt, Priority};
pub use coalesced::Coalesced;
pub use priority::{Multiple, Posterior, Prior, Single};
