//! Provides constants.

use crate::Variables;

/// Plane Vanilla Plus.
pub const VANILLA_PLUS: Variables = include!("vanilla-plus.jetlab");

/// Optimization rate.
pub const OPTIMIZATION_RATE: f64 = 0.01;

/// Minimum allowable thrust.
pub const MIN_THRUST: f64 = 80067.989;

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

/// Limitations on gradient ascent optimizations.
pub const INLET_MACH_NUMBER_MIN: f64                = 0.20;
pub const INLET_MACH_NUMBER_MAX: f64                = 0.67;
pub const INLET_DIAMETER_MIN: f64                   = 1.2192;
pub const INLET_DIAMETER_MAX: f64                   = 1.3716;
pub const FAN_PRESSURE_RATIO_MIN: f64               = 1.0;
pub const FAN_PRESSURE_RATIO_MAX: f64               = 1.8;
pub const FAN_BYPASS_MIN: f64                       = 1.0;
pub const FAN_BYPASS_MAX: f64                       = 13.0;
pub const LPC_PRESSURE_RATIO_MIN: f64               = 1.0;
pub const LPC_PRESSURE_RATIO_MAX: f64               = 3.0;
pub const HPC_PRESSURE_RATIO_MIN: f64               = 1.0;
pub const HPC_PRESSURE_RATIO_MAX: f64               = 22.5;
pub const HPC_DISCHARGE_TEMP_MIN: f64               = 0.0;
pub const HPC_DISCHARGE_TEMP_MAX: f64               = 1527.594;
pub const HPT_INLET_TEMP_MIN: f64                   = 0.0;
pub const HPT_INLET_TEMP_MAX: f64                   = 2005.372;