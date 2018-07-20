#![feature(proc_macro, specialization)]

extern crate pyo3;
extern crate libosu;

use pyo3::prelude::*;
use pyo3::py::{class as pyclass, methods as pymethods};

#[pyclass]
pub struct Beatmap<'a> {
    internal: libosu::Beatmap<'a>,
}

#[pymethods]
impl<'a> Beatmap<'a> {
    #[staticmethod]
    pub fn from_string(input: String) -> PyResult<()> {
    }
}
