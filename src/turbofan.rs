//! Computes the performance of a turbofan.

use crate::{
    constants::*,
    plot,
    Variables,
    VarSelector,
};

#[derive(Clone, Copy, Debug)]
/// Constructs a turbofan.
pub struct Turbofan { }

impl Turbofan {
    /// Constructs a new turbofan.
    pub fn new() -> Self {
        Self { }
    }

    /// Computes the thrust and SFC of this engine.
    pub fn analyze(&self, variables: Variables) -> (f64, f64) {
        let t_t1 = T0;
        let p_t1 = P0;
        let a1 = variables.inlet_area();
        let m1 = variables.inlet_mach_number;

        // Compute mass flow rate
        let mdot = p_t1*a1*t_t1.powf(-0.5) * (GAMMA / R).sqrt() * m1 * (1.0 + (GAMMA - 1.0)/2.0 * m1.powi(2)).powf((GAMMA + 1.0)/(2.0*(1.0 - GAMMA)));

        // Fan
        let t_t2 = t_t1;
        let p_t2 = p_t1;
        let eta = variables.fan_efficiency;
        let pi = variables.fan_pressure_ratio;
        let t_t23 = t_t2 * (1.0 + 1.0/eta * (pi.powf((GAMMA - 1.0)/GAMMA) - 1.0));
        let p_t23 = pi*p_t2;

        // Low pressure compressor
        let eta = variables.lpc_efficiency;
        let pi = variables.lpc_pressure_ratio;
        let t_t25 = t_t23 * (1.0 + 1.0/eta * (pi.powf((GAMMA - 1.0)/GAMMA) - 1.0));
        let p_t25 = pi*p_t23;
        
        // High pressure compressor
        let eta = variables.hpc_efficiency;
        let pi = variables.hpc_pressure_ratio;
        let t_t3 = t_t25 * (1.0 + 1.0/eta * (pi.powf((GAMMA - 1.0)/GAMMA) - 1.0));
        let p_t3 = pi*p_t25;

        // Combustor
        let t_t4 = variables.hpt_inlet_temp;
        let cp_fuel = variables.fuel_cp;
        let f = cp_fuel * (t_t4 - t_t3) / variables.fuel_delta_h;
        let p_t4 = p_t3;

        // High pressure turbine
        let eta = variables.hpt_efficiency;
        let t_t45 = t_t4 - CP_AIR/cp_fuel/(1.0 + f) * (t_t3 - t_t25);
        let p_t45 = p_t4 * (1.0 - 1.0/eta * (1.0 - t_t45/t_t4)).powf(GAMMA_FLUE/(GAMMA_FLUE - 1.0));

        // Low pressure turbine
        let eta = variables.lpt_efficiency;
        let beta = variables.fan_bypass;
        let t_t5 = t_t45 - CP_AIR*(1.0 + beta)/cp_fuel/(1.0 + f) * (t_t23 - t_t2) - CP_AIR/cp_fuel/(1.0 + f) * (t_t25 - t_t23);
        let p_t5 = p_t45 * (1.0 - 1.0/eta * (1.0 - t_t5/t_t45)).powf(GAMMA_FLUE/(GAMMA_FLUE - 1.0));

        // Bypass nozzle
        let p_t19 = p_t23;
        let t_t19 = t_t23;
        let m19 = (2.0 / (GAMMA - 1.0) * ((p_t19 / P0).powf((GAMMA - 1.0)/GAMMA) - 1.0)).sqrt();
        let t19 = t_t19 / (1.0 + (GAMMA - 1.0)/2.0 * m19.powi(2));
        let v19 = m19 * (GAMMA * R * t19).sqrt();

        // Core nozzle
        let p_t9 = p_t5;
        let t_t9 = t_t5;
        let m9 = (2.0 / (GAMMA_FLUE - 1.0) * ((p_t9 / P0).powf((GAMMA_FLUE - 1.0)/GAMMA_FLUE) - 1.0)).sqrt();
        let t9 = t_t9 / (1.0 + (GAMMA_FLUE - 1.0)/2.0 * m9.powi(2));
        let v9 = m9 * (GAMMA_FLUE * R * t9).sqrt();

        // Thrust
        let thrust = mdot * ((1.0 + f)/(1.0 + beta)*v9 + beta/(1.0 + beta) * v19);

        // SFC
        let sfc = f/(1.0 + beta) * mdot/thrust * 3600.0;

        (thrust, sfc)
    }

    /// Plot thrust as a function of one variable.
    pub fn plot_thrust(
        selected: VarSelector,
        left: f64,
        right: f64,
    ) {
        use VarSelector::*;
        let n: usize = 1000;

        let function = match selected {
            InletMachNumber             => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_mach_number = var;
                Self::new().analyze(variables).0
            },
            InletDiameter               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_diameter = var;
                Self::new().analyze(variables).0
            },
            InletEfficiency             => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_efficiency = var;
                Self::new().analyze(variables).0
            },
            FanPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_pressure_ratio = var;
                Self::new().analyze(variables).0
            },
            FanEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_efficiency = var;
                Self::new().analyze(variables).0
            },
            FanBypass                   => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_bypass = var;
                Self::new().analyze(variables).0
            },
            LpcPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpc_pressure_ratio = var;
                Self::new().analyze(variables).0
            },
            LpcEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpc_efficiency = var;
                Self::new().analyze(variables).0
            },
            HpcPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_pressure_ratio = var;
                Self::new().analyze(variables).0
            },
            HpcEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_efficiency = var;
                Self::new().analyze(variables).0
            },
            HpcDischargeTemp            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_discharge_temp = var;
                Self::new().analyze(variables).0
            },
            CombustorPressureRecovery   => |var| {
                let mut variables = VANILLA_PLUS;
                variables.combustor_pressure_recovery = var;
                Self::new().analyze(variables).0
            },
            CombustorEfficiency         => |var| {
                let mut variables = VANILLA_PLUS;
                variables.combustor_efficiency = var;
                Self::new().analyze(variables).0
            },
            HptInletTemp                => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpt_inlet_temp = var;
                Self::new().analyze(variables).0
            },
            HptEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpt_efficiency = var;
                Self::new().analyze(variables).0
            },
            LptEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpt_efficiency = var;
                Self::new().analyze(variables).0
            },
            BypassPressureRecovery      => |var| {
                let mut variables = VANILLA_PLUS;
                variables.bypass_pressure_recovery = var;
                Self::new().analyze(variables).0
            },
            FuelDeltaH                  => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fuel_delta_h = var;
                Self::new().analyze(variables).0
            },
            FuelCp                      => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fuel_cp = var;
                Self::new().analyze(variables).0
            },
        };

        let varname: String = selected.into();

        plot(
            function,
            left,
            right,
            n,
            &varname,
            "Thrust (N)",
            &format!("{} Thrust Plot.png", varname),
        );
    }

    /// Plot specific fuel consumption as a function of one variable.
    pub fn plot_sfc(
        selected: VarSelector,
        left: f64,
        right: f64,
    ) {
        use VarSelector::*;
        let n: usize = 1000;

        let function = match selected {
            InletMachNumber             => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_mach_number = var;
                Self::new().analyze(variables).1
            },
            InletDiameter               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_diameter = var;
                Self::new().analyze(variables).1
            },
            InletEfficiency             => |var| {
                let mut variables = VANILLA_PLUS;
                variables.inlet_efficiency = var;
                Self::new().analyze(variables).1
            },
            FanPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_pressure_ratio = var;
                Self::new().analyze(variables).1
            },
            FanEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_efficiency = var;
                Self::new().analyze(variables).1
            },
            FanBypass                   => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fan_bypass = var;
                Self::new().analyze(variables).1
            },
            LpcPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpc_pressure_ratio = var;
                Self::new().analyze(variables).1
            },
            LpcEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpc_efficiency = var;
                Self::new().analyze(variables).1
            },
            HpcPressureRatio            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_pressure_ratio = var;
                Self::new().analyze(variables).1
            },
            HpcEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_efficiency = var;
                Self::new().analyze(variables).1
            },
            HpcDischargeTemp            => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpc_discharge_temp = var;
                Self::new().analyze(variables).1
            },
            CombustorPressureRecovery   => |var| {
                let mut variables = VANILLA_PLUS;
                variables.combustor_pressure_recovery = var;
                Self::new().analyze(variables).1
            },
            CombustorEfficiency         => |var| {
                let mut variables = VANILLA_PLUS;
                variables.combustor_efficiency = var;
                Self::new().analyze(variables).1
            },
            HptInletTemp                => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpt_inlet_temp = var;
                Self::new().analyze(variables).1
            },
            HptEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.hpt_efficiency = var;
                Self::new().analyze(variables).1
            },
            LptEfficiency               => |var| {
                let mut variables = VANILLA_PLUS;
                variables.lpt_efficiency = var;
                Self::new().analyze(variables).1
            },
            BypassPressureRecovery      => |var| {
                let mut variables = VANILLA_PLUS;
                variables.bypass_pressure_recovery = var;
                Self::new().analyze(variables).1
            },
            FuelDeltaH                  => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fuel_delta_h = var;
                Self::new().analyze(variables).1
            },
            FuelCp                      => |var| {
                let mut variables = VANILLA_PLUS;
                variables.fuel_cp = var;
                Self::new().analyze(variables).1
            },
        };

        let varname: String = selected.into();

        plot(
            function,
            left,
            right,
            n,
            &varname,
            "Specific Fuel Consumption (kg/N-hr)",
            &format!("{} SFC Plot.png", varname),
        );
    }
}