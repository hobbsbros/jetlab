//! Allows the user to select for a single variable.

use std::process;

#[derive(Clone, Copy, Debug)]
/// Enumerates the variables the user may select.
pub enum VarSelector {
    InletMachNumber,
    InletDiameter,
    InletEfficiency,
    FanPressureRatio,
    FanEfficiency,
    FanBypass,
    LpcPressureRatio,
    LpcEfficiency,
    HpcPressureRatio,
    HpcEfficiency,
    HpcDischargeTemp,
    CombustorPressureRecovery,
    CombustorEfficiency,
    HptInletTemp,
    HptEfficiency,
    LptEfficiency,
    BypassPressureRecovery,
    FuelDeltaH,
    FuelCp,
}

use VarSelector::*;

impl From<VarSelector> for String {
    fn from(var: VarSelector) -> Self {
        let string = match var {
            InletMachNumber             => "Inlet Mach Number",
            InletDiameter               => "Inlet Diameter",
            InletEfficiency             => "Inlet Efficiency",
            FanPressureRatio            => "Fan Pressure Ratio",
            FanEfficiency               => "Fan Efficiency",
            FanBypass                   => "Fan Bypass Ratio",
            LpcPressureRatio            => "LPC Pressure Ratio",
            LpcEfficiency               => "LPC Efficiency",
            HpcPressureRatio            => "HPC Pressure Ratio",
            HpcEfficiency               => "HPC Efficiency",
            HpcDischargeTemp            => "HPC Discharge Temperature",
            CombustorPressureRecovery   => "Combustor Pressure Recovery Coefficient",
            CombustorEfficiency         => "Combustor Efficiency",
            HptInletTemp                => "HPT Inlet Temperature",
            HptEfficiency               => "HPT Efficiency",
            LptEfficiency               => "LPT Efficiency",
            BypassPressureRecovery      => "Bypass Pressure Recovery Coefficient",
            FuelDeltaH                  => "Fuel Enthalpy",
            FuelCp                      => "Fuel CP",
        };

        string.to_string()
    }
}

impl From<String> for VarSelector {
    fn from(string: String) -> Self {
        match string.as_str() {
            "inlet_mach_number" => InletMachNumber,
            "inlet_diameter" => InletDiameter,
            "inlet_efficiency" => InletEfficiency,
            "fan_pressure_ratio" => FanPressureRatio,
            "fan_efficiency" => FanEfficiency,
            "fan_bypass" => FanBypass,
            "lpc_pressure_ratio" => LpcPressureRatio,
            "lpc_efficiency" => LpcEfficiency,
            "hpc_pressure_ratio" => HpcPressureRatio,
            "hpc_efficiency" => HpcEfficiency,
            "hpc_discharge_temp" => HpcDischargeTemp,
            "combustor_pressure_recovery" => CombustorPressureRecovery,
            "combustor_efficiency" => CombustorEfficiency,
            "hpt_inlet_temp" => HptInletTemp,
            "hpt_efficiency" => HptEfficiency,
            "lpt_efficiency" => LptEfficiency,
            "bypass_pressure_recovery" => BypassPressureRecovery,
            "fuel_delta_h" => FuelDeltaH,
            "fuel_cp" => FuelCp,
            _ => {
                println!("Invalid variable name.  Type `jetlab` for help.");
                process::exit(0);
            },
        }
    }
}