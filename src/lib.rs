mod coalesced;
mod extension;
mod priority;

#[cfg(feature = "clap")]
mod clap;
#[cfg(feature = "serde")]
mod serde;

pub use coalesced::{Coalesce, Coalesced};
pub use priority::{Multiple, Posterior, Prior, Single};
