pub mod dim2;
pub mod dim3;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests;

use std::f32::consts::PI;

pub type Unit = f32;
pub const UNIT_PI: Unit = PI;
pub const EPSILON: Unit = 0.0001;

pub type IUnit = i32;
