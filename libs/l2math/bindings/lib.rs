//! For Documentation please refer [LLM](../llm)

#![cfg_attr(feature = "no_std", no_std)]

#[doc(hidden)]
pub use l2math::*;

#[no_mangle]
#[panic_handler]
#[rustfmt::skip]
#[cfg(feature = "no_std")]
pub extern "C" fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
