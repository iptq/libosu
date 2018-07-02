#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
extern crate libosu;

mod beatmap;

use pyo3::prelude::*;
use pyo3::py::modinit;

pub use beatmap::*;

/// libosu is an attempt to make a convenient library for writing osu!-related programs. It
/// includes data structures and parsers for beatmaps, replays, and more.
#[modinit(libosu)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Beatmap>()?;
    Ok(())
}
