//! Main executable for the Jetlab.

use jetlab::{
    Cli,
    Turbofan,
};

fn main() {
    // Read user input
    let cli = Cli::new();

    // Set up a turbofan
    let fan = Turbofan::new();

    if let Cli::Plot {
        variable,
        left,
        right,
        allvars,
    } = cli {
        let variables = allvars;

        // Plot thrust
        fan.plot_thrust(
            variable,
            left,
            right,
            variables,
        );
        
        // Plot SFC
        fan.plot_sfc(
            variable,
            left,
            right,
            variables,
        );
    } else if let Cli::Optimize {
        allvars,
        steps,
    } = cli {
        // let thrust_opt = fan.optimize_thrust(allvars, steps);
        let sfc_opt = fan.optimize_sfc(allvars, steps);

        println!("OPTIMIZATION RESULTS");
        println!("====================\n");

        // println!("Thrust focus");
        // println!("{}\n", thrust_opt);

        println!("SFC focus");
        println!("{}", sfc_opt);
    }
}