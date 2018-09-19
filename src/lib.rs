//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing osu!-related programs. It
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! Please note that until this crate hits `1.0`, none of the APIs in this crate will be stable, so
//! take care when using this crate.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate num_rational;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod beatmap;
mod color;
mod enums;
mod hitobject;
mod hitsounds;
mod osz;
mod point;
mod timing;

pub use beatmap::*;
pub use color::*;
pub use enums::*;
pub use hitobject::*;
pub use hitsounds::*;
pub use osz::*;
pub use point::*;
pub use timing::*;

/// Says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
