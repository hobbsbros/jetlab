//! Main library for the Jetlab.

mod cli;
pub mod constants;
mod turbofan;
mod plot;
mod variables;
mod varselect;

pub use cli::Cli;
pub use plot::plot;
pub use turbofan::Turbofan;
pub use variables::Variables;
pub use varselect::VarSelector;