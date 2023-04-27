//! Main executable for the Jetlab.

use jetlab::{
    constants::*,
    Turbofan,
    // Variables,
};

fn main() {
    let fan = Turbofan::new();
    let default = VANILLA_PLUS;

    let (thrust, sfc) = fan.analyze(default);

    println!("Thrust: {:.6} N", thrust);
    println!("SFC: {:.6} kg / h-N", sfc);
}