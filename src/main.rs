//! Main executable for the Jetlab.

use jetlab::{
    Cli,
    constants::*,
    Turbofan,
};

fn main() {
    // Read user input
    let cli = Cli::new();

    // Plot thrust
    Turbofan::plot_thrust(
        cli.variable,
        cli.left,
        cli.right,
        cli.allvars,
    );
    
    // Plot SFC
    Turbofan::plot_sfc(
        cli.variable,
        cli.left,
        cli.right,
        cli.allvars,
    );
}

#[allow(dead_code)]
fn analyze_baseline() {
    let fan = Turbofan::new();
    let default = VANILLA_PLUS;
    let (thrust, sfc) = fan.analyze(default);
    println!("BASELINE");
    println!("========");
    println!("Thrust: {:.6} N", thrust);
    println!("SFC: {:.6} kg / h-N", sfc);
}