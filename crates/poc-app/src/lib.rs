#![no_std]

use poc_adapter::Platform;
use poc_core::{AppEvent, Error, Fault, MqttState, Result, TimestampMs};
use poc_service::DeviceState;

pub struct App<const OUTPUTS: usize> {
    state: DeviceState<OUTPUTS>,
}

impl<const OUTPUTS: usize> App<OUTPUTS> {
    pub const fn new() -> Self {
        Self {
            state: DeviceState::new(),
        }
    }

    pub fn dispatch(&mut self, platform: &mut impl Platform, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Boot { timestamp } => self.on_boot(platform, timestamp),
            AppEvent::Tick { timestamp } => self.on_tick(platform, timestamp),
            AppEvent::MqttConnected { timestamp } => self.on_mqtt_connected(platform, timestamp),
            AppEvent::MqttDisconnected { timestamp } => {
                self.on_mqtt_disconnect(platform, timestamp)
            }
            AppEvent::InputChanged { snapshot } => {
                self.state.update_inputs(snapshot);
                Ok(())
            }
        }
    }

    pub fn state(&self) -> &DeviceState<OUTPUTS> {
        &self.state
    }

    fn on_boot(&mut self, platform: &mut impl Platform, _timestamp: TimestampMs) -> Result<()> {
        platform.log_info("poc app boot")?;
        self.state.mark_running();
        Ok(())
    }

    fn on_tick(&mut self, platform: &mut impl Platform, timestamp: TimestampMs) -> Result<()> {
        platform.feed_watchdog()?;

        match platform.read_inputs() {
            Ok(snapshot) => self.state.update_inputs(snapshot),
            Err(_) => {
                self.state.mark_faulted(Fault::Platform);
                return Err(Error::Io);
            }
        }

        match platform.read_temperature() {
            Ok(sample) => self.state.update_temperature(sample),
            Err(_) => {
                self.state.mark_faulted(Fault::Platform);
                return Err(Error::Io);
            }
        }

        if self.state.should_publish_telemetry(timestamp) {
            self.publish_telemetry(platform)?;
            self.state.mark_telemetry_published(timestamp);
        }

        Ok(())
    }

    fn on_mqtt_connected(
        &mut self,
        platform: &mut impl Platform,
        _timestamp: TimestampMs,
    ) -> Result<()> {
        self.state.set_mqtt(MqttState::Connected);
        platform.log_info("mqtt connectd")?;
        self.publish_telemetry(platform)
    }

    fn on_mqtt_disconnect(
        &mut self,
        platform: &mut impl Platform,
        _timestamp: TimestampMs,
    ) -> Result<()> {
        self.state.set_mqtt(MqttState::Disconnected);
        platform.log_info("mqtt disconnected")
    }

    fn publish_telemetry(&mut self, platform: &mut impl Platform) -> Result<()> {
        platform.publish_status(b"{\"status\":\"ok\"}")
    }
}

impl<const OUTPUTS: usize> Default for App<OUTPUTS> {
    fn default() -> Self {
        Self::new()
    }
}
