//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing OSU-related programs. It
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! Please note that until this crate hits `1.0`, none of the APIs in this crate will be stable, so
//! take care when using this crate. Always pin to the version that you are using!

#![deny(missing_docs)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

pub(crate) mod float;

/// Client for the OSU api
#[cfg(feature = "apiv1")]
pub mod apiv1;
/// Beatmaps
pub mod beatmap;
/// Defines the Color struct
pub mod color;

/// Deals with osu database files (osu.db, collections.db, etc)
pub mod db;
/// Data structures
// TODO: should probably split this and move enums into their respective modules
pub mod enums;
/// Errors
pub mod errors;
/// Beatmap events
pub mod events;
/// Hit-objects
pub mod hitobject;
/// Data structures for hitsounds
pub mod hitsounds;
/// Math
pub mod math;
// /// Working with beatmap files.
// pub mod parsing;
/// Working with replays.
#[cfg(feature = "replay")]
pub mod replay;
/// Calculating slider body shapes.
pub mod spline;
/// Timing and timing points.
pub mod timing;

/// Exports everything in the library.
pub mod prelude {
    #[cfg(feature = "apiv1")]
    pub use crate::apiv1::*;
    pub use crate::beatmap::*;
    pub use crate::color::*;
    pub use crate::db::*;
    pub use crate::enums::*;
    pub use crate::events::*;
    pub use crate::hitobject::*;
    pub use crate::hitsounds::*;
    pub use crate::math::*;
    #[cfg(feature = "replay")]
    pub use crate::replay::*;
    pub use crate::spline::*;
    pub use crate::timing::*;
}

/// Says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
