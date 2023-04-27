//! Enables command-line interface for the Jetlab executable.

use std::{
    env,
    process,
};

use crate::{
    constants::*,
    Variables,
    VarSelector,
};

/// Holds command-line inputs.
pub struct Cli {
    pub variable: VarSelector,
    pub left: f64,
    pub right: f64,
    pub allvars: Variables,
}

impl Cli {
    /// Reads input from CLI.
    pub fn new() -> Self {
        let args = env::args().collect::<Vec<String>>();

        if args.len() < 4 {
            Self::help();
        }

        let variable: VarSelector = args[1].clone().into();
        
        let left = match str::parse::<f64>(&args[2]) {
            Ok (f) => f,
            Err (_) => {
                println!("[FATAL] Could not parse input {}", args[2].clone());
                process::exit(0);
            }
        };
        let right = match str::parse::<f64>(&args[3]) {
            Ok (f) => f,
            Err (_) => {
                println!("[FATAL] Could not parse input {}", args[3].clone());
                process::exit(0);
            }
        };

        // Initialize variables
        let mut allvars = VANILLA_PLUS;

        let mut i = 4;
        while i < args.len() {
            let arg = &args[i];

            if arg == "--fix" {
                i += 1;
                let arg = &args[i];

                // Stop when you find another flag
                while !(&args[i]).starts_with("--") {
                    let option: VarSelector = args[i].clone().into();

                    if args.len() == i + 1 {
                        println!("[FATAL] Please specify a value for {}", arg);
                        process::exit(0);
                    }

                    let value = match str::parse::<f64>(&args[i + 1]) {
                        Ok (f) => f,
                        Err (_) => {
                            println!("[FATAL] Could not parse {} as numeric value", &args[i + 1]);
                            process::exit(0);
                        },
                    };

                    *allvars.get_reference(option) = value;

                    i += 2;
                    if i >= args.len() {
                        break;
                    }
                }
            } else {
                println!("[ERROR] Unrecognized flag {}.  Type `jetlab` for help.", arg).
            }

            i += 1;
        }

        Self {
            variable,
            left,
            right,
            allvars,
        }
    }

    /// Provides a help menu to the user.
    pub fn help() -> ! {
        println!("JETLAB");

        println!("\nUsage:\n\t$ jetlab [VARIABLE] [LOWER_BOUND] [UPPER_BOUND]");
        println!("\t$ jetlab [VARIABLE] [LOWER_BOUND] [UPPER_BOUND] --fix [VARIABLE] [VALUE]");

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
}