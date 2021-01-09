#![allow(dead_code)]

pub const FLOAT_ERROR_32: f32 = 0.001;
pub const FLOAT_ERROR_64: f64 = 0.001;

pub fn compare_eq_f32(a: f32, b: f32) -> bool {
    (a - b).abs() < FLOAT_ERROR_32
}

pub fn compare_eq_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < FLOAT_ERROR_64
}
