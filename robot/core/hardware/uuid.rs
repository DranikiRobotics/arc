/// A Unique Hardware Identifier
/// 
/// This is a struct that is used to identify a specific piece of hardware.
/// This is can be used to load a hardware component from the hardware map.
/// 
/// This is a wrapper around `[char; 8]`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareUUID([char; 8]);

impl HardwareUUID {
    /// Creates a new HardwareUUID from a `[char; 8]`
    #[inline(always)]
    pub const fn new(uuid: [char; 8]) -> Self {
        Self(uuid)
    }
    /// Converts something that can be converted into a string into a HardwareUUID
    /// 
    /// ### Note
    /// 
    /// If the string is less than 8 characters, the rest of the UUID will be filled with `0`s
    #[inline]
    pub fn from_to_string(input: impl ToString) -> Self {
        let mut uuid = ['0'; 8];
        input.to_string()
            .chars()
            .enumerate()
            .for_each(|(i, c)| uuid[i] = c);
        Self::new(uuid)
    }
}

impl From<[char; 8]> for HardwareUUID {
    #[inline(always)]
    fn from(uuid: [char; 8]) -> Self {
        Self::new(uuid)
    }
}

impl From<String> for HardwareUUID {
    /// Converts a string into a HardwareUUID
    /// 
    /// ### Note
    /// 
    /// If the string is less than 8 characters, the rest of the UUID will be filled with `0`s
    #[inline]
    fn from(input: String) -> Self {
        Self::from_to_string(input)
    }
}

impl<'a> From<&'a str> for HardwareUUID {
    /// Converts a `str` into a HardwareUUID
    /// 
    /// ### Note
    /// 
    /// If the string is less than 8 characters, the rest of the UUID will be filled with `0`s
    #[inline]
    fn from(input: &'a str) -> Self {
        Self::from_to_string(input)
    }
}

impl From<HardwareUUID> for [char; 8] {
    #[inline(always)]
    fn from(uuid: HardwareUUID) -> Self {
        uuid.0
    }
}

impl From<HardwareUUID> for String {
    #[inline(always)]
    fn from(uuid: HardwareUUID) -> Self {
        uuid.0.iter().collect()
    }
}

impl ToString for HardwareUUID {
    #[inline(always)]
    fn to_string(&self) -> String {
        self.0.iter().collect()
    }
}
