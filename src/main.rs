//! Main executable for the Jetlab.

use jetlab::{
    Cli,
    constants::*,
    Turbofan,
};

fn main() {
    let fan = Turbofan::new();
    let default = VANILLA_PLUS;
    let (thrust, sfc) = fan.analyze(default);
    println!("BASELINE");
    println!("========");
    println!("Thrust: {:.6} N", thrust);
    println!("SFC: {:.6} kg / h-N", sfc);
    println!("\n");

    let cli = Cli::new();

    Turbofan::plot_thrust(
        cli.variable,
        cli.left,
        cli.right,
    );
}