#![doc = include_str!("../README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod gamepad;

/// ### O_O
///
/// A type that can be used to represent a result that is always `Ok`
///
/// It is a shorthand for `Result<(), HardwareError>`.
pub const IO_OK: Result = Ok(());

/// An error that occurs when reading from or writing to a hardware component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareError {
    /// An IO error occurred
    ///
    /// This error is returned when reading from or writing to a hardware component fails.
    IO,
    /// Also an IO error, but this one is returned when the hardware component is disconnected.
    ///
    /// That means that the hardware component was connected when the program started, but it was
    /// later disconnected.
    Disconnected,
    /// Some other error occurred
    Other {
        /// The error message
        message: &'static str,
    },
}

/// A result that occurs when reading or writing to a hardware component
pub type Result<T = (), E = HardwareError> = core::result::Result<T, E>;

impl core::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IO => write!(f, "IO error"),
            Self::Disconnected => write!(f, "Gamepad disconnected"),
            Self::Other { message } => write!(f, "{}", message),
        }
    }
}

impl From<&'static str> for HardwareError {
    #[inline]
    fn from(message: &'static str) -> Self {
        Self::Other { message }
    }
}

impl From<HardwareError> for String {
    #[inline]
    fn from(error: HardwareError) -> Self {
        format!("{}", error)
    }
}

impl From<HardwareError> for &'static str {
    #[inline]
    fn from(error: HardwareError) -> Self {
        match error {
            HardwareError::IO => "IO error",
            HardwareError::Disconnected => "Gamepad disconnected",
            HardwareError::Other { message } => message,
        }
    }
}

#[allow(unsafe_code)]
unsafe impl Send for HardwareError {}
#[allow(unsafe_code)]
unsafe impl Sync for HardwareError {}

impl std::error::Error for HardwareError {}
