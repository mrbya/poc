use poc_core::{InputSnapshot, OutputId, OutputState, Result, TemperatureSample, TimestampMs};

pub trait Platform {
    fn uptime_ms(&mut self) -> TimestampMs;

    fn log_info(&mut self, message: &str) -> Result<()>;

    fn set_output(&mut self, output: OutputId, state: OutputState) -> Result<()>;

    fn read_inputs(&mut self) -> Result<InputSnapshot>;

    fn read_temperature(&mut self) -> Result<TemperatureSample>;

    fn publish_status(&mut self, payload: &[u8]) -> Result<()>;

    fn feed_watchdog(&mut self) -> Result<()>;
}

pub mod zephyr;
