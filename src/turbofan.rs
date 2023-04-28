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

    /// Compute the partial derivative of the thrust at a given value with respect to a given variable.
    pub fn delta_thrust(&self, variables: Variables, var: VarSelector) -> f64 {
        let dx: f64 = 1.0E-6;

        let mut variables_high = variables;
        *variables_high.get_reference(var) += dx;

        let mut variables_low = variables;
        *variables_low.get_reference(var) -= dx;

        (self.analyze(variables_high).0 - self.analyze(variables_low).0)/(2.0*dx)
    }

    /// Compute the partial derivative of the SFC at a given value with respect to a given variable.
    pub fn delta_sfc(&self, variables: Variables, var: VarSelector) -> f64 {
        let dx = 0.001;

        let mut variables_high = variables;
        *variables_high.get_reference(var) += dx;

        let mut variables_low = variables;
        *variables_low.get_reference(var) -= dx;

        (self.analyze(variables_high).1 - self.analyze(variables_low).1)/(2.0*dx)
    }

    /// Compute the gradient of the thrust value with respect to all variables.
    pub fn thrust_gradient(&self, variables: Variables) -> Variables {
        use VarSelector::*;

        Variables {
            inlet_mach_number: self.delta_thrust(variables, InletMachNumber),
            inlet_diameter: self.delta_thrust(variables, InletDiameter),
            inlet_efficiency: 0.0,
            fan_pressure_ratio: self.delta_thrust(variables, FanPressureRatio),
            fan_efficiency: 0.0,
            fan_bypass: self.delta_thrust(variables, FanBypass),
            lpc_pressure_ratio: self.delta_thrust(variables, LpcPressureRatio),
            lpc_efficiency: 0.0,
            hpc_pressure_ratio: self.delta_thrust(variables, HpcPressureRatio),
            hpc_efficiency: 0.0,
            hpc_discharge_temp: self.delta_thrust(variables, HpcDischargeTemp),
            combustor_pressure_recovery: 0.0,
            combustor_efficiency: 0.0,
            hpt_inlet_temp: self.delta_thrust(variables, HptInletTemp),
            hpt_efficiency: 0.0,
            lpt_efficiency: 0.0,
            bypass_pressure_recovery: 0.0,
            fuel_delta_h: 0.0,
            fuel_cp: 0.0,
        }
    }

    /// Compute the gradient of the SFC value with respect to all variables.
    pub fn sfc_gradient(&self, variables: Variables) -> Variables {
        use VarSelector::*;

        Variables {
            inlet_mach_number: self.delta_sfc(variables, InletMachNumber),
            inlet_diameter: self.delta_sfc(variables, InletDiameter),
            inlet_efficiency: 0.0,
            fan_pressure_ratio: self.delta_sfc(variables, FanPressureRatio),
            fan_efficiency: 0.0,
            fan_bypass: self.delta_sfc(variables, FanBypass),
            lpc_pressure_ratio: self.delta_sfc(variables, LpcPressureRatio),
            lpc_efficiency: 0.0,
            hpc_pressure_ratio: self.delta_sfc(variables, HpcPressureRatio),
            hpc_efficiency: 0.0,
            hpc_discharge_temp: self.delta_sfc(variables, HpcDischargeTemp),
            combustor_pressure_recovery: 0.0,
            combustor_efficiency: 0.0,
            hpt_inlet_temp: self.delta_sfc(variables, HptInletTemp),
            hpt_efficiency: 0.0,
            lpt_efficiency: 0.0,
            bypass_pressure_recovery: 0.0,
            fuel_delta_h: 0.0,
            fuel_cp: 0.0,
        }
    }

    /// Perform a gradient ascent optimization step for maximizing thrust.
    fn step_thrust_optimization(&self, variables: Variables) -> Variables {
        let step = self.thrust_gradient(variables).mult(OPTIMIZATION_RATE);
        
        variables + step
    }

    /// Perform a gradient ascent optimization step for minimizing SFC.
    fn step_sfc_optimization(&self, variables: Variables) -> Variables {
        let step = self.sfc_gradient(variables).mult(OPTIMIZATION_RATE);
        
        variables + step.mult(-1.0)
    }

    /// Perform gradient ascent optimization to maximize thrust.
    pub fn optimize_thrust(&self, mut variables: Variables, n: usize) -> Variables {
        for _ in 0..n {
            variables = self.step_thrust_optimization(variables);
        }

        variables
    }

    /// Perform gradient ascent optimization to minimize SFC.
    pub fn optimize_sfc(&self, mut variables: Variables) -> Variables {
        #[allow(unused_assignments)]
        let (mut t, mut sfc) = (MIN_THRUST, 0.0);
        while t >= MIN_THRUST {
            variables = self.step_sfc_optimization(variables);
            (t, sfc) = self.analyze(variables);
            println!("Thrust: {:.8} N | SFC: {:.8} kg/N-hr", t, sfc);
        }

        variables
    }

    /// Plot thrust as a function of one variable.
    pub fn plot_thrust(
        &self,
        selected: VarSelector,
        left: f64,
        right: f64,
        vars: Variables,
    ) {
        let n: usize = 1000;

        let thrust = |input| {
            let mut variables = vars;
            *variables.get_reference(selected) = input;
            Turbofan::new().analyze(variables).0
        };

        let varname: String = selected.into();

        plot(
            thrust,
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
        &self,
        selected: VarSelector,
        left: f64,
        right: f64,
        vars: Variables,
    ) {
        let n: usize = 1000;

        let sfc = |input| {
            let mut variables = vars;
            *variables.get_reference(selected) = input;
            Turbofan::new().analyze(variables).1
        };

        let varname: String = selected.into();

        plot(
            sfc,
            left,
            right,
            n,
            &varname,
            "SFC (kg/N-hr)",
            &format!("{} SFC Plot.png", varname),
        );
    }
}