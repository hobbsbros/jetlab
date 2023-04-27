//! Allows the user to select for a single variable.

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