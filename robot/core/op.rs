use crate::prelude::*;
use crate::threadsafe::*;
use crate::{Result, HardwareUUID};

#[cfg(not(feature = "only_builtins"))]
#[derive(Debug, Clone)]
pub struct Op<H, T, G> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
{
    running: ThreadSafeBool,
    start_time: std::time::Instant,
    hardware_map: ThreadSafe<H>,
    telemetry: ThreadSafe<T>,
    gamepad: ThreadSafe<G>,
}

#[cfg(feature = "only_builtins")]
#[derive(Debug, Clone)]
pub struct Op {
    running: ThreadSafeBool,
    start_time: std::time::Instant,
    hardware_map: ThreadSafe<crate::internals::builtins::BuiltInHardwareMapImpl>,
    telemetry: ThreadSafe<crate::internals::builtins::BuiltInTelemetryImpl>,
    gamepad: ThreadSafe<crate::internals::builtins::BuiltInGamepadImpl>,
}

#[cfg(feature = "only_builtins")]
impl Default for Op {
    #[inline(always)]
    fn default() -> Self {
        Self::new(
            crate::internals::builtins::BuiltInHardwareMapImpl::default(),
            crate::internals::builtins::BuiltInTelemetryImpl::default(),
            crate::internals::builtins::BuiltInGamepadImpl::default()
        )
    }
}

use crate::internals::impls;
use impls::HardwareMapImpl as BuiltInHardwareMapImpl;
use impls::TelemetryImpl as BuiltInTelemetryImpl;
use impls::GamepadImpl as BuiltInGamepadImpl;

#[cfg(not(feature = "only_builtins"))]
pub type RuntimeOp<
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
> = ThreadSafe<Op<H, T, G>>;

#[cfg(feature = "only_builtins")]
pub type RuntimeOp = ThreadSafe<Op>;

#[cfg(not(feature = "only_builtins"))]
impl<H, T, G> Op<H, T, G> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
{
    /// Creates a new `Op` with the given hardware map, telemetry, and gamepad
    #[inline(always)]
    #[must_use = "This returns a new Op"]
    pub fn new(hardware_map: H, telemetry: T, gamepad: G) -> Self {
        Self {
            hardware_map: ThreadSafe::new(hardware_map),
            telemetry: ThreadSafe::new(telemetry),
            gamepad: ThreadSafe::new(gamepad),
            running: ThreadSafeBool::new(false),
            start_time: std::time::Instant::now()
        }
    }

    /// Returns if the op mode is running
    /// 
    /// If the mutex is poisoned, this will return false
    #[inline]
    pub fn running(&self) -> bool {
        match self.running.get() {
            Ok(r) => **r,
            Err(_) => false,
        }
    }
    /// Returns if the op mode is running
    /// 
    /// The result is wrapped in a `Result` because
    /// internally, the value is wrapped in a `Mutex`
    /// 
    /// If the [`Mutex`] is poisoned, this will return an error
    /// 
    /// [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
    #[inline]
    pub fn running_result(&self) -> Result<bool> {
        Ok(self.running.get().map(|r| **r)?)
    }
    /// Returns a reference to the [`HardwareMap`] of the op mode.
    /// 
    /// It is mostly used for loading hardware components.
    /// However, you may use this struct to load hardware components as well.
    /// 
    /// See [`HardwareMap`] for more information.
    /// 
    /// [`HardwareMap`]: ./trait.HardwareMap.html
    #[inline]
    pub fn hardware_map(&self) -> GetResult<'_, H> {
        self.hardware_map.get()
    }
    /// Returns a cloned version of the [`HardwareMap`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`HardwareMap`] for more information.
    /// 
    /// [`HardwareMap`]: ./trait.HardwareMap.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_hardware_map(&self) -> ThreadSafe<H> {
        self.hardware_map.clone()
    }
    /// Returns the [`Telemetry`] of the op mode.
    /// 
    /// It is mostly used for sending log messages to the driver control station.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    #[inline]
    pub fn telemetry(&self) -> GetResult<'_, T> {
        self.telemetry.get()
    }
    /// Returns a cloned version of the [`Telemetry`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_telemetry(&self) -> ThreadSafe<T> {
        self.telemetry.clone()
    }
    /// Returns the [`Gamepad`] of the op mode.
    /// 
    /// Used for controlling the in TeleOp Modes.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    #[inline]
    pub fn gamepad(&self) -> GetResult<'_, G> {
        self.gamepad.get()
    }
    /// Returns a cloned version of the [`Gamepad`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_gamepad(&self) -> ThreadSafe<G> {
        self.gamepad.clone()
    }
}

#[cfg(feature = "only_builtins")]
impl Op {
    pub fn init() -> RuntimeOp {
        let hardware_map = crate::internals::builtins::BuiltInHardwareMapImpl::default();
        let telemetry = crate::internals::builtins::BuiltInTelemetryImpl::default();
        let gamepad = crate::internals::builtins::BuiltInGamepadImpl::default();
        let op = Self::new(hardware_map, telemetry, gamepad);
        ThreadSafe::new(op)
    }

    /// Creates a new `Op` with the given hardware map, telemetry, and gamepad
    #[inline(always)]
    #[must_use = "This returns a new Op"]
    pub fn new(
        hardware_map: BuiltInHardwareMapImpl,
        telemetry: BuiltInTelemetryImpl,
        gamepad: BuiltInGamepadImpl
    ) -> Self {
        Self {
            hardware_map: ThreadSafe::new(hardware_map),
            telemetry: ThreadSafe::new(telemetry),
            gamepad: ThreadSafe::new(gamepad),
            running: ThreadSafeBool::new(false),
            start_time: std::time::Instant::now()
        }
    }

    /// Returns if the op mode is running
    /// 
    /// If the mutex is poisoned, this will return false
    #[inline]
    pub fn running(&self) -> bool {
        match self.running.get() {
            Ok(r) => **r,
            Err(_) => false,
        }
    }
    /// Returns if the op mode is running
    /// 
    /// The result is wrapped in a `Result` because
    /// internally, the value is wrapped in a `Mutex`
    /// 
    /// If the [`Mutex`] is poisoned, this will return an error
    /// 
    /// [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
    #[inline]
    pub fn running_result(&self) -> Result<bool> {
        Ok(self.running.get().map(|r| **r)?)
    }
    /// Returns a reference to the [`HardwareMap`] of the op mode.
    /// 
    /// It is mostly used for loading hardware components.
    /// However, you may use this struct to load hardware components as well.
    /// 
    /// See [`HardwareMap`] for more information.
    /// 
    /// [`HardwareMap`]: ./trait.HardwareMap.html
    #[inline]
    pub fn hardware_map(&self) -> GetResult<'_, BuiltInHardwareMapImpl> {
        self.hardware_map.get()
    }
    /// Returns a cloned version of the [`HardwareMap`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`HardwareMap`] for more information.
    /// 
    /// [`HardwareMap`]: ./trait.HardwareMap.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_hardware_map(&self) -> ThreadSafe<BuiltInHardwareMapImpl> {
        self.hardware_map.clone()
    }
    /// Returns the [`Telemetry`] of the op mode.
    /// 
    /// It is mostly used for sending log messages to the driver control station.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    #[inline]
    pub fn telemetry(&self) -> GetResult<'_, BuiltInTelemetryImpl> {
        self.telemetry.get()
    }
    /// Returns a cloned version of the [`Telemetry`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Telemetry`] for more information.
    /// 
    /// [`Telemetry`]: ./trait.Telemetry.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_telemetry(&self) -> ThreadSafe<BuiltInTelemetryImpl> {
        self.telemetry.clone()
    }
    /// Returns the [`Gamepad`] of the op mode.
    /// 
    /// Used for controlling the in TeleOp Modes.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    #[inline]
    pub fn gamepad(&self) -> GetResult<'_, BuiltInGamepadImpl> {
        self.gamepad.get()
    }
    /// Returns a cloned version of the [`Gamepad`] of the op mode.
    /// 
    /// This doesn't do bit for bit cloning, but instead, it clones the
    /// internal [`Arc`] that holds the data.
    /// 
    /// See [`Gamepad`] for more information.
    /// 
    /// [`Gamepad`]: ./trait.Gamepad.html
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    #[inline]
    pub fn get_gamepad(&self) -> ThreadSafe<BuiltInGamepadImpl> {
        self.gamepad.clone()
    }
}

#[cfg(not(feature = "only_builtins"))]
impl<H, T, G> HardwareMap for Op<H, T, G> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad,
{
    /// Loads a hardware component with the given UUID
    /// 
    /// This is just a shortcut for `self.hardware_map.load(uuid)`
    #[inline]
    fn load<C: crate::hardware::HardwareComponent>(&self, uuid: impl Into<HardwareUUID>) -> Result<C> {
        self.hardware_map.get()?.load(uuid)
    }
}

#[cfg(feature = "only_builtins")]
impl HardwareMap for Op {
    /// Loads a hardware component with the given UUID
    /// 
    /// This is just a shortcut for `self.hardware_map.load(uuid)`
    #[inline]
    fn load<C: crate::hardware::HardwareComponent>(&self, uuid: impl Into<HardwareUUID>) -> Result<C> {
        self.hardware_map()?.load(uuid)
    }
}
