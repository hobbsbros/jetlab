//! Main library for the Jetlab.

pub mod constants;
mod turbofan;
mod variables;

pub use turbofan::Turbofan;
pub use variables::Variables;