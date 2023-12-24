#![doc = include_str!("../README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod hardware;
pub mod gamepad;
mod telemetry;
mod error;

pub use error::{HardwareError, Result, IO_OK};
pub use hardware::HardwareMap;
pub use telemetry::Telemetry;

use gamepad::Gamepad;

#[derive(Debug, Clone)]
pub struct OpMode<H, T, G> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
{
    hardware_map: H,
    telemetry: T,
    gamepad: G,
}

impl<H, T, G> OpMode<H, T, G> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
{
    /// Creates a new `OpMode` with the given hardware map, telemetry, and gamepad
    #[inline(always)]
    #[must_use = "This returns a new OpMode"]
    pub const fn new(hardware_map: H, telemetry: T, gamepad: G) -> Self {
        Self { hardware_map, telemetry, gamepad }
    }
}

#[path = "hardware/uuid.rs"]
mod __uuid;
pub use __uuid::HardwareUUID;

/// A trait that represents a hardware component
pub trait HardwareComponent: core::fmt::Debug {
    /// Returns the UUID of this component
    #[allow(non_snake_case)]
    fn getUUID(&self) -> HardwareUUID;
}
