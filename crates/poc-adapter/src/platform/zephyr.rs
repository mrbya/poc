use crate::{
    feed_watchdog, log_info, platform::Platform, publish_status, read_inputs, read_temperature,
    set_output, uptime_ms,
};

pub struct ZephyrPlatform;

impl ZephyrPlatform {
    pub const fn new() -> Self {
        Self
    }
}

impl Platform for ZephyrPlatform {
    fn uptime_ms(&mut self) -> poc_core::TimestampMs {
        uptime_ms()
    }

    fn log_info(&mut self, message: &str) -> poc_core::Result<()> {
        log_info(message)
    }

    fn set_output(
        &mut self,
        _output: poc_core::OutputId,
        _state: poc_core::OutputState,
    ) -> poc_core::Result<()> {
        set_output()
    }

    fn read_inputs(&mut self) -> poc_core::Result<poc_core::InputSnapshot> {
        read_inputs()
    }

    fn read_temperature(&mut self) -> poc_core::Result<poc_core::TemperatureSample> {
        read_temperature()
    }

    fn publish_status(&mut self, payload: &[u8]) -> poc_core::Result<()> {
        publish_status(payload)
    }

    fn feed_watchdog(&mut self) -> poc_core::Result<()> {
        feed_watchdog()
    }
}

impl Default for ZephyrPlatform {
    fn default() -> Self {
        Self::new()
    }
}
