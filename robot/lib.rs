#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

/// Helps with loading robot configurations.
/// 
/// ## Example using the default config
/// 
/// ```rust
/// dranik::use_config!();
/// dranik::main!();
/// ```
/// 
/// ## Example using ARC
/// 
/// ```rust
/// dranik::use_config!(arc);
/// dranik::main!();
/// ```
#[macro_export(local_inner_macros)]
macro_rules! use_config {
    () => ( $crate::use_config!($crate); );
    ($namespace: path) => (
        use $namespace::{__dranik_config as __DranikRobotConfig};
    );
}

/// Creates the main function for the robot.
/// 
/// ## Example
/// 
/// ```rust
/// dranik::use_config!();
/// dranik::main!();
/// ```
/// 
/// This macro is used to create the main function for the robot.
/// It is guaranteed to be stable and it's API will not change.
/// 
/// The reason this macro exists is because the main function
/// is not guaranteed to be stable and may change at any time.
/// That mostly includes the generic parameters.
#[macro_export(local_inner_macros)]
macro_rules! main {
    () => ( fn main() {
        $crate::main::<__DranikRobotConfig>();
    } );
}

pub use dranikcore::prelude::RobotConfig;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use tokio::runtime::Builder;

static ID: AtomicU8 = AtomicU8::new(0);

/// This is the actual main function that is called by the robot.
/// 
/// It isn't recommended to call this function directly.
/// However, if you do, be careful as this function is not
/// guaranteed to be stable and may change at any time.
/// See [`main!`] for more information.
/// 
/// ## Recommended Usage
/// 
/// ```rust
/// dranik::use_config!();
/// dranik::main!();
/// ```
/// 
/// ## Example using ARC
/// 
/// ```rust
/// dranik::use_config!(arc);
/// dranik::main!();
/// ```
/// 
/// ## Internal workings
/// 
/// What it does in a nutshell is:
/// 1. Initialize the async runtime.
/// 2. Setup the IO.
/// 3. Spawn the web server.
/// 4. Spawn the python thread.
/// 5. inject shutdown handler.
/// 6. Block until shutdown.
/// 7. Shutdown the runtime.
/// 8. Exit.
/// 
/// [`main!`]: crate::main
pub fn main<C: RobotConfig + 'static>() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name_fn(|| {
            let id = ID.fetch_add(1, Ordering::SeqCst);
            format!("Dranik worker#{}", id)
        })
        .build()
        .expect("Failed to initialize runtime");

    // This isn't how it's supposed to be done, but it works for now
    //
    // In the future the http server will be the one creating the OpMode
    // and then feeding it to the Python thread.
    let op = dranikcore::Op::init();

    // runtime.spawn(dranikweb::main());
    runtime.spawn(dranikpy::main::<C::Args, C>(op));
    runtime.block_on(async {
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
        print!("Exiting...");
    });
    runtime.shutdown_timeout(std::time::Duration::from_secs(5));
    std::process::exit(0);
}

/// Internal struct that is used to hold the robot config.
/// 
/// This contains the default config.
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub struct __dranik_config;
impl dranikcore::prelude::RobotConfig for __dranik_config {
    type Args = ();
}
