#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[cfg(any(
    all(
        feature = "only_builtins",
        feature = "allow_external_impls"
    ),
    all(
        not(feature = "only_builtins"),
        not(feature = "allow_external_impls")
    )
))]
compile_error!("The `only_builtins` and `allow_external_impls` features are mutually exclusive.");

#[cfg(not(feature = "reveal_modules"))]
mod threadsafe;
#[cfg(not(feature = "reveal_modules"))]
mod hardware;
#[cfg(not(feature = "reveal_modules"))]
mod gamepad;
#[cfg(feature = "reveal_modules")]
pub mod threadsafe;
#[cfg(feature = "reveal_modules")]
pub mod hardware;
#[cfg(feature = "reveal_modules")]
pub mod gamepad;

#[cfg(all(
    feature = "reveal_modules",
    feature = "internals"
))]
#[doc(hidden)]
pub mod internals;
#[cfg(not(all(
    feature = "reveal_modules",
    feature = "internals"
)))]
mod internals;

mod telemetry;
mod config;
mod error;
mod op;

/// The prelude for the `arc` crate.
/// 
/// This prelude re-exports all of the important types and traits
pub mod prelude {
    pub use super::hardware::*;
    pub use super::config::RobotConfig;
    pub use super::gamepad::{Gamepad, MutableGamepad};
    pub use super::telemetry::Telemetry;
    pub use super::threadsafe::ThreadSafeError;
    pub use super::thread_safe;
}

pub use op::{Op, RuntimeOp};
pub use error::{HardwareError, Result, IO_OK};

#[path = "hardware/uuid.rs"]
mod __uuid;
pub use __uuid::HardwareUUID;

#[doc(hidden)]
type DeblockResult<T> = core::result::Result<T, tokio::task::JoinError>;

/// Deblocks a blocking piece of code.
/// 
/// This function is used to take a blocking piece of code and run it in such a way
/// that it doesn't block the entire runtime.
pub async fn deblock<F, R>(f: F) -> DeblockResult<R> where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f).await
}
