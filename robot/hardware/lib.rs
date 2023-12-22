#![doc = include_str!("../README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod gamepad;
mod error;

pub use error::*;

#[derive(Debug, Clone)]
pub struct HardwareMap {

}

impl HardwareMap {
    pub fn init() -> Self {
        Self {
        }
    }
}
