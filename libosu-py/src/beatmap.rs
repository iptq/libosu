use pyo3::prelude::*;
use pyo3::py::class as pyclass;
use pyo3::py::methods as pymethods;

use libosu;

/// Describes a single osu! Beatmap.
#[pyclass]
pub struct Beatmap { 
    #[prop(get)]
    val: usize,
}

