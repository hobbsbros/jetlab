//! Provides constants.

use crate::Variables;

/// Plane Vanilla Plus.
pub const VANILLA_PLUS: Variables = include!("vanilla-plus.jetlab");

/// Ratio of specific heats for air.
pub const GAMMA: f64 = 1.400;

/// Ratio of specific heats for flue gas.
pub const GAMMA_FLUE: f64 = 1.333;

/// Specific gas constant for air.
pub const R: f64 = 287.00;

/// Free stream temperature.
pub const T0: f64 = 288.15;

/// Free stream pressure.
pub const P0: f64 = 101325.353;

/// Specific heat capacity of air at constant pressure.
pub const CP_AIR: f64 = 1004.50;

/// Pi.
pub const PI: f64 = 3.141549265358979323846264;
