#![no_std]

use poc_core::{
    Fault, InputSnapshot, LifecycleState, MqttState, OutputState, TemperatureSample, TimestampMs,
};

pub struct DeviceState<const OUTPUTS: usize> {
    pub lifecycle: LifecycleState,
    pub mqtt: MqttState,
    pub outputs: [OutputState; OUTPUTS],
    pub inputs: InputSnapshot,
    pub temperature: TemperatureSample,
    pub telemetry_interval_ms: u64,
    pub last_telemetry_at: TimestampMs,
}

impl<const OUTPUTS: usize> DeviceState<OUTPUTS> {
    pub const fn new() -> Self {
        Self {
            lifecycle: LifecycleState::Booting,
            mqtt: MqttState::Disconnected,
            outputs: [OutputState::Inactive; OUTPUTS],
            inputs: InputSnapshot::EMPTY,
            temperature: TemperatureSample::INVALID,
            telemetry_interval_ms: 1000,
            last_telemetry_at: TimestampMs::ZERO,
        }
    }
    pub fn mark_running(&mut self) {
        self.lifecycle = LifecycleState::Running;
    }

    pub fn mark_faulted(&mut self, fault: Fault) {
        self.lifecycle = LifecycleState::Faulted(fault);
    }

    pub fn set_mqtt(&mut self, state: MqttState) {
        self.mqtt = state;
    }

    pub fn update_inputs(&mut self, snapshot: InputSnapshot) {
        self.inputs = snapshot;
    }

    pub fn update_temperature(&mut self, sample: TemperatureSample) {
        self.temperature = sample;
    }

    pub fn should_publish_telemetry(&self, now: TimestampMs) -> bool {
        now.elapsed_since(self.last_telemetry_at) >= self.telemetry_interval_ms
    }

    pub fn mark_telemetry_published(&mut self, now: TimestampMs) {
        self.last_telemetry_at = now;
    }
}

impl<const OUTPUTS: usize> Default for DeviceState<OUTPUTS> {
    fn default() -> Self {
        Self::new()
    }
}
