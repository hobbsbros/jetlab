//! Abstracts over optimization variables.

use crate::{
    constants::*,
    VarSelector,
};

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

use VarSelector::*;

impl Variables {
    /// Computes inlet area (in m2).
    pub fn inlet_area(&self) -> f64 {
        0.25 * PI * self.inlet_diameter.powi(2)
    }

    /// Gets a mutable reference to a field based on a `VarSelector`.
    pub fn get_reference(&mut self, selection: VarSelector) -> &mut f64 {
        match selection {
            InletMachNumber             => &mut self.inlet_mach_number,
            InletDiameter               => &mut self.inlet_diameter,
            InletEfficiency             => &mut self.inlet_efficiency,
            FanPressureRatio            => &mut self.fan_pressure_ratio,
            FanEfficiency               => &mut self.fan_efficiency,
            FanBypass                   => &mut self.fan_bypass,
            LpcPressureRatio            => &mut self.lpc_pressure_ratio,
            LpcEfficiency               => &mut self.lpc_efficiency,
            HpcPressureRatio            => &mut self.hpc_pressure_ratio,
            HpcEfficiency               => &mut self.hpc_efficiency,
            HpcDischargeTemp            => &mut self.hpc_discharge_temp,
            CombustorPressureRecovery   => &mut self.combustor_pressure_recovery,
            CombustorEfficiency         => &mut self.combustor_efficiency,
            HptInletTemp                => &mut self.hpt_inlet_temp,
            HptEfficiency               => &mut self.hpt_efficiency,
            LptEfficiency               => &mut self.lpt_efficiency,
            BypassPressureRecovery      => &mut self.bypass_pressure_recovery,
            FuelDeltaH                  => &mut self.fuel_delta_h,
            FuelCp                      => &mut self.fuel_cp,
        }
    }
}