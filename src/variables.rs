//! Abstracts over optimization variables.

use std::{
    fmt::{
        Display,
        Formatter,
        Result,
    },
    ops::{
        Add,
    },
};

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

    /// Applies a constant multiplier to each value.
    pub fn mult(&self, k: f64) -> Self {
        Self {
            inlet_mach_number: k*self.inlet_mach_number,
            inlet_diameter: k*self.inlet_diameter,
            inlet_efficiency: k*self.inlet_efficiency,
            fan_pressure_ratio: k*self.fan_pressure_ratio,
            fan_efficiency: k*self.fan_efficiency,
            fan_bypass: k*self.fan_bypass,
            lpc_pressure_ratio: k*self.lpc_pressure_ratio,
            lpc_efficiency: k*self.lpc_efficiency,
            hpc_pressure_ratio: k*self.hpc_pressure_ratio,
            hpc_efficiency: k*self.hpc_efficiency,
            hpc_discharge_temp: k*self.hpc_discharge_temp,
            combustor_pressure_recovery: k*self.combustor_pressure_recovery,
            combustor_efficiency: k*self.combustor_efficiency,
            hpt_inlet_temp: k*self.hpt_inlet_temp,
            hpt_efficiency: k*self.hpt_efficiency,
            lpt_efficiency: k*self.lpt_efficiency,
            bypass_pressure_recovery: k*self.bypass_pressure_recovery,
            fuel_delta_h: k*self.fuel_delta_h,
            fuel_cp: k*self.fuel_cp,
        }
    }
}

impl Display for Variables {
    /// Format this data structure.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut output = String::new();

        output.push_str(&format!("Inlet Mach number: {:.6}\n", self.inlet_mach_number));
        output.push_str(&format!("Inlet diameter: {:.6} m\n", self.inlet_diameter));
        output.push_str(&format!("Inlet efficiency: {:.6}%\n", self.inlet_efficiency*100.0));
        output.push_str(&format!("Fan pressure ratio: {:.6}\n", self.fan_pressure_ratio));
        output.push_str(&format!("Fan efficiency: {:.6}%\n", self.fan_efficiency*100.0));
        output.push_str(&format!("Fan bypass ratio: {:.6}\n", self.fan_bypass));
        output.push_str(&format!("LPC pressure ratio: {:.6}\n", self.lpc_pressure_ratio));
        output.push_str(&format!("LPC efficiency: {:.6}%\n", self.lpc_efficiency*100.0));
        output.push_str(&format!("HPC pressure ratio: {:.6}\n", self.hpc_pressure_ratio));
        output.push_str(&format!("HPC efficiency: {:.6}%\n", self.inlet_mach_number*100.0));
        output.push_str(&format!("HPC discharge temperature: {:.6} K\n", self.hpc_discharge_temp));
        output.push_str(&format!("Combustor pressure recovery coefficient: {:.6}\n", self.combustor_pressure_recovery));
        output.push_str(&format!("Combustor efficiency: {:.6}%\n", self.combustor_efficiency*100.0));
        output.push_str(&format!("HPT inlet temperature: {:.6} K\n", self.hpt_inlet_temp));
        output.push_str(&format!("HPT efficiency: {:.6}%\n", self.hpt_efficiency*100.0));
        output.push_str(&format!("LPT efficiency: {:.6}%\n", self.lpt_efficiency*100.0));
        output.push_str(&format!("Bypass duct pressure recovery coefficient: {:.6}\n", self.bypass_pressure_recovery));
        output.push_str(&format!("Fuel enthalpy: {:.6} J/kg\n", self.fuel_delta_h));
        output.push_str(&format!("Fuel specific heat capacity at constant pressure: {:.6} J/kg-K\n", self.fuel_cp));

        write!(f, "{}", output)
    }
}

impl Add for Variables {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            inlet_mach_number: (self.inlet_mach_number + other.inlet_mach_number).clamp(INLET_MACH_NUMBER_MIN, INLET_MACH_NUMBER_MAX),
            inlet_diameter: (self.inlet_diameter + other.inlet_diameter).clamp(INLET_DIAMETER_MIN, INLET_DIAMETER_MAX),
            inlet_efficiency: self.inlet_efficiency + other.inlet_efficiency,
            fan_pressure_ratio: (self.fan_pressure_ratio + other.fan_pressure_ratio).clamp(FAN_PRESSURE_RATIO_MIN, FAN_PRESSURE_RATIO_MAX),
            fan_efficiency: self.fan_efficiency + other.fan_efficiency,
            fan_bypass: (self.fan_bypass + other.fan_bypass).clamp(FAN_BYPASS_MIN, FAN_BYPASS_MAX),
            lpc_pressure_ratio: (self.lpc_pressure_ratio + other.lpc_pressure_ratio).clamp(LPC_PRESSURE_RATIO_MIN, LPC_PRESSURE_RATIO_MAX),
            lpc_efficiency: self.lpc_efficiency + other.lpc_efficiency,
            hpc_pressure_ratio: (self.hpc_pressure_ratio + other.hpc_pressure_ratio).clamp(HPC_PRESSURE_RATIO_MIN, HPC_PRESSURE_RATIO_MAX),
            hpc_efficiency: self.hpc_efficiency + other.hpc_efficiency,
            hpc_discharge_temp: (self.hpc_discharge_temp + other.hpc_discharge_temp).clamp(HPC_DISCHARGE_TEMP_MIN, HPC_DISCHARGE_TEMP_MAX),
            combustor_pressure_recovery: self.combustor_pressure_recovery + other.combustor_pressure_recovery,
            combustor_efficiency: self.combustor_efficiency + other.combustor_efficiency,
            hpt_inlet_temp: (self.hpt_inlet_temp + other.hpt_inlet_temp).clamp(HPT_INLET_TEMP_MIN, HPT_INLET_TEMP_MAX),
            hpt_efficiency: self.hpt_efficiency + other.hpt_efficiency,
            lpt_efficiency: self.lpt_efficiency + other.lpt_efficiency,
            bypass_pressure_recovery: self.bypass_pressure_recovery + other.bypass_pressure_recovery,
            fuel_delta_h: self.fuel_delta_h + other.fuel_delta_h,
            fuel_cp: self.fuel_cp + other.fuel_cp,
        }
    }
}