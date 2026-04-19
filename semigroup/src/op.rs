mod coalesce;
mod concat;
mod first;
mod last;
pub use {coalesce::*, concat::*, first::*, last::*};

mod all;
mod any;
#[cfg(feature = "monoid")]
mod gcd;
mod intersection;
#[cfg(feature = "monoid")]
mod lcm;
mod max;
mod min;
mod prod;
mod sum;
mod union;
mod xor;
pub use {all::*, any::*, intersection::*, max::*, min::*, prod::*, sum::*, union::*, xor::*};
#[cfg(feature = "monoid")]
pub use {gcd::*, lcm::*};

#[cfg(feature = "histogram")]
mod hdr_histogram;
#[cfg(feature = "histogram")]
pub use hdr_histogram::*;
