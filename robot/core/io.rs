use crate::threadsafe::{ThreadSafe, GetResult, SafeHeld, StandardResult};
use crate::*;

#[derive(Debug)]
struct IOHolder {
    gamepad: internals::impls::GamepadImpl,
}
impl IOHolder {
    fn init() -> Self {
        Self {
            gamepad: internals::impls::GamepadImpl::init(),
        }
    }
    fn gamepad(&self) -> impl Gamepad {
        self.gamepad.clone()
    }
}

#[derive(Debug, Clone)]
pub struct IO(ThreadSafe<IOHolder>);

impl IO {
    pub(crate) fn new() -> Self {
        Self(ThreadSafe::new(IOHolder::init()))
    }
    pub fn gamepad(&self) -> StandardResult<impl Gamepad> {
        self.0.get().map(|h| h.gamepad())
    }
    
}
