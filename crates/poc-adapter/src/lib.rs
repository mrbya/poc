#![no_std]

pub(crate) mod clock;
pub(crate) mod ffi;
pub(crate) mod gpio;
pub(crate) mod logger;
pub(crate) mod mqtt;
pub(crate) mod platform;
pub(crate) mod temperature;

// Re-exports
pub use ffi::{
    feed_watchdog, log_info, publish_status, read_inputs, read_temperature, set_output,
    start_tick_runtime, uptime_ms, PocBytes, PocStatus,
};
pub use platform::zephyr::ZephyrPlatform;
pub use platform::Platform;
