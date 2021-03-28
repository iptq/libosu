//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing osu-related programs. it
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! please note that until this crate hits `1.0`, none of the apis in this crate will be stable, so
//! take care when using this crate. always pin to the version that you are using!

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate thiserror;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

/// provides notnan
pub extern crate ordered_float;

pub(crate) mod float;
pub(crate) mod utils;

/// client for the osu api
#[cfg(feature = "apiv1")]
#[cfg_attr(docsrs, doc(cfg(feature = "apiv1")))]
pub mod apiv1;
#[cfg(feature = "apiv2")]
#[cfg_attr(docsrs, doc(cfg(feature = "apiv2")))]
pub mod apiv2;
/// beatmaps
pub mod beatmap;
/// defines the color struct
pub mod color;

/// deals with osu database files (osu.db, collections.db, etc)
pub mod db;
/// data structures
pub mod data;
/// errors
pub mod errors;
/// beatmap events
pub mod events;
/// hit-objects
pub mod hitobject;
/// data structures for hitsounds
pub mod hitsounds;
/// math
pub mod math;
pub mod replay;
/// calculating slider body shapes.
pub mod spline;
/// timing and timing points.
pub mod timing;

/// exports everything in the library.
pub mod prelude {
    #[cfg(feature = "apiv1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "apiv1")))]
    pub use crate::apiv1::*;
    #[cfg(feature = "apiv2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "apiv2")))]
    pub use crate::apiv2::*;
    pub use crate::beatmap::{diff_calc::*, pp_calc::*, *};
    pub use crate::color::*;
    pub use crate::db::*;
    pub use crate::data::*;
    pub use crate::events::*;
    pub use crate::hitobject::*;
    pub use crate::hitsounds::*;
    pub use crate::math::*;
    pub use crate::replay::*;
    pub use crate::spline::*;
    pub use crate::timing::*;
    pub use ordered_float::*;
}

/// says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
