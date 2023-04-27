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

        if args.len() < 4 {
            println!("JETLAB");
            println!("\nUsage:\n\t$ jetlab [VARIABLE] [LOWER_BOUND] [UPPER_BOUND]");
            println!("\nFree Variables:");
            println!("\tinlet_mach_number");
            println!("\tinlet_diameter");
            println!("\tinlet_efficiency");
            println!("\tfan_pressure_ratio");
            println!("\tfan_efficiency");
            println!("\tfan_bypass");
            println!("\tlpc_pressure_ratio");
            println!("\tlpc_efficiency");
            println!("\thpc_pressure_ratio");
            println!("\thpc_efficiency");
            println!("\thpc_discharge_temp");
            println!("\tcombustor_pressure_recovery");
            println!("\tcombustor_efficiency");
            println!("\thpt_inlet_temp");
            println!("\thpt_efficiency");
            println!("\tlpt_efficiency");
            println!("\tbypass_pressure_recovery");
            println!("\tfuel_delta_h");
            println!("\tfuel_cp");
            process::exit(0);
        }

        let variable: VarSelector = args[1].clone().into();
        
        let left = match str::parse::<f64>(&args[2]) {
            Ok (f) => f,
            Err (_) => {
                println!("Could not parse input {}", args[2].clone());
                process::exit(0);
            }
        };
        let right = match str::parse::<f64>(&args[3]) {
            Ok (f) => f,
            Err (_) => {
                println!("Could not parse input {}", args[3].clone());
                process::exit(0);
            }
        };

        Self {
            variable,
            left,
            right,
        }
    }
}