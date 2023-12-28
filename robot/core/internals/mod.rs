#![doc = include_str!("./INTERNALS.md")]

use super::*;

pub(crate) mod impls;

/// Metadata used to load a hardware component
#[derive(Debug, Clone)]
pub struct HardwareComponentLoadMetadata {
    pub uuid: HardwareUUID,
}
