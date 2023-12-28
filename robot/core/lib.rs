#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod threadsafe;
pub mod hardware;
pub mod gamepad;
pub mod config;
pub mod io;

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

#[doc(hidden)]
pub mod internals;

pub fn setup_io() -> io::IO {
    io::IO::new()
}

/// This function is used to take a blocking piece of code and run it in such a way
/// that it doesn't block the entire runtime.
pub async fn deblock<F, R>(f: F) -> core::result::Result<R, tokio::task::JoinError> where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f).await
}
