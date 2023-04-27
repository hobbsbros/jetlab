//! Provides constants.

use crate::Variables;

/// Plane Vanilla Plus.
pub const VANILLA_PLUS: Variables = Variables {
    inlet_mach_number:              0.4,
    inlet_diameter:                 1.27,
    inlet_efficiency:               0.98,
    fan_pressure_ratio:             1.5,
    fan_efficiency:                 0.87,
    fan_bypass:                     6.0,
    lpc_pressure_ratio:             2.0,
    lpc_efficiency:                 0.87,
    hpc_pressure_ratio:             8.0,
    hpc_efficiency:                 0.83,
    hpc_discharge_temp:             922.0389,
    combustor_pressure_recovery:    0.98,
    combustor_efficiency:           0.99,
    hpt_inlet_temp:                 1527.594,
    hpt_efficiency:                 0.87,
    lpt_efficiency:                 0.89,
    bypass_pressure_recovery:       0.98,
    fuel_delta_h:                   43E+6,
    fuel_cp:                        1148.4941,
};

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
