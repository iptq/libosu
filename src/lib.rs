//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing osu!-related programs. It
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! Please note that until this crate hits `1.0`, none of the APIs in this crate will be stable, so
//! take care when using this crate.

#![deny(missing_docs)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

#[cfg(feature = "apiv1")]
mod apiv1;
mod beatmap;
mod color;
mod enums;
mod hitobject;
mod hitsounds;
mod parsing;
mod point;
#[cfg(feature = "replay")]
mod replay;
mod timing;

#[cfg(feature = "apiv1")]
pub use apiv1::{APIError, ApprovedStatus, API, UserScore, UserLookup};
pub use beatmap::*;
pub use color::*;
pub use enums::*;
pub use hitobject::*;
pub use hitsounds::*;
pub use parsing::*;
pub use point::*;
#[cfg(feature = "replay")]
pub use replay::*;
pub use timing::*;

/// Says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
