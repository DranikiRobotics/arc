/// A type that can be used to represent a result that is always `Ok`
/// 
/// ### O_O
///
/// It is a shorthand for `Result<(), HardwareError>`.
pub const IO_OK: Result = Ok(());

/// An error that occurs when reading from or writing to a hardware component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareError {
    /// The device was disconnected
    DeviceDisconnected,
    /// The device was not found
    DeviceNotFound,
    /// Some other error occurred
    Other {
        /// The error message
        message: &'static str,
    },
}

impl HardwareError {
    /// Creates a new `HardwareError::Other` with the given message
    #[inline]
    #[must_use = "This returns a new HardwareError"]
    pub const fn new(message: &'static str) -> Self {
        Self::Other { message }
    }
    /// Returns the error message
    #[inline]
    #[must_use = "This returns a new string slice"]
    pub const fn as_str(&self) -> &'static str {
        match self {
            HardwareError::DeviceDisconnected => "Device disconnected",
            HardwareError::DeviceNotFound => "Device not found",
            HardwareError::Other { message } => message,
        }
    }
}

/// A result that occurs when reading or writing to a hardware component
pub type Result<T = (), E = HardwareError> = core::result::Result<T, E>;

impl core::fmt::Display for HardwareError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
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
        error.as_str()
    }
}

#[allow(unsafe_code)]
unsafe impl Send for HardwareError {}
#[allow(unsafe_code)]
unsafe impl Sync for HardwareError {}

impl std::error::Error for HardwareError {}
