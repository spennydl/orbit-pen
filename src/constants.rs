use std::f64;

pub const UNIT: f64 = 385000600.0; // earth-moon distance in m
pub const SCALE: f64 = 250.0 / UNIT; // 400px / unit
pub const TIMESTEP: f64 = 3600.0; // one hour
pub const PI: f64 = 3.1415926535;
pub const G: f64 = 0.0000000000667408;