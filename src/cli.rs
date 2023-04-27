//! Enables command-line interface for the Jetlab executable.

use std::{
    env,
    process,
};

use crate::VarSelector;

/// Holds command-line inputs.
pub struct Cli {
    pub variable: VarSelector,
    pub left: f64,
    pub right: f64,
}

impl Cli {
    /// Reads input from CLI.
    pub fn new() -> Self {
        let args = env::args().collect::<Vec<String>>();

        use VarSelector::*;

        if args.len() != 4 {
            println!("\n\nJETLAB");
            println!("\nUsage:\n\t$ jetlab [VARIABLE] [LOWER_BOUND] [UPPER_BOUND]");
            process::exit(0);
        }

        let variable = match args[1].as_str() {
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
            _ => todo!(),
        };
        
        let left = str::parse::<f64>(&args[2]).unwrap();
        let right = str::parse::<f64>(&args[3]).unwrap();

        Self {
            variable,
            left,
            right,
        }
    }
}