mod coalesced;
mod extension;
mod priority;

#[cfg(feature = "clap")]
mod clap;
#[cfg(feature = "serde")]
mod serde;

pub use coalesced::Coalesced;
pub use priority::{Multiple, Posterior, Prior, Single};

pub trait Coalesce {
    fn straight(&self, other: &Self) -> bool;
}
impl<T> Coalesce for Option<T> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(_), _) => true,
            _ => false,
        }
    }
}
impl<T, E> Coalesce for Result<T, E> {
    fn straight(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(_), _) => true,
            _ => false,
        }
    }
}
