//! Abstracts over optimization variables.

use crate::constants::*;

#[derive(Clone, Copy, Debug)]
/// Holds all engine optimization variables.
pub struct Variables {
    /// Inlet Mach number
    pub inlet_mach_number: f64,

    /// Inlet diameter (in m)
    pub inlet_diameter: f64,

    /// Inlet efficiency
    pub inlet_efficiency: f64,

    /// Fan pressure ratio
    pub fan_pressure_ratio: f64,

    /// Fan adiabatic efficiency
    pub fan_efficiency: f64,

    /// Fan bypass ratio
    pub fan_bypass: f64,

    /// Low pressure compressor pressure ratio
    pub lpc_pressure_ratio: f64,

    /// Low pressure compressor adiabatic efficiency
    pub lpc_efficiency: f64,

    /// High pressure compressor pressure ratio
    pub hpc_pressure_ratio: f64,

    /// High pressure compressor adiabatic efficiency
    pub hpc_efficiency: f64,

    /// High pressure compressor discharge temperature (in K)
    pub hpc_discharge_temp: f64,

    /// Combustor pressure recovery coefficient
    pub combustor_pressure_recovery: f64,

    /// Combustor efficiency
    pub combustor_efficiency: f64,

    /// High pressure turbine inlet temperature
    pub hpt_inlet_temp: f64,

    /// High pressure turbine efficiency
    pub hpt_efficiency: f64,

    /// Low pressure turbine efficiency
    pub lpt_efficiency: f64,

    /// Bypass duct pressure recovery coefficient
    pub bypass_pressure_recovery: f64,

    /// Fuel enthalpy of combustion
    pub fuel_delta_h: f64,

    /// Fuel specific heat capacity at constant pressure
    pub fuel_cp: f64,
}

impl Variables {
    /// Computes inlet area (in m2).
    pub fn inlet_area(&self) -> f64 {
        0.25 * PI * self.inlet_diameter.powi(2)
    }
}