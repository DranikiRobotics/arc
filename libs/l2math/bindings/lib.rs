//! For Documentation please refer [LLM](../llm)

#![cfg_attr(feature = "no_std", no_std)]

#[doc(hidden)]
pub use l2math::*;

#[doc(hidden)]
#[warn(dead_code)]
#[allow(non_snake_case)]
#[cfg(not(feature = "no_std"))]
fn Relax_this_is_not_an_actual_warning_This_is_here_to_inform_you_that_this_crate_is_not_being_built_with_no_std_() {}
