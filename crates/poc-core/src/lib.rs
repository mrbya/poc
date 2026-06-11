#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Io,
    InvalidArgument,
    Timeout,
    Busy,
    Internal,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimestampMs(pub u64);

impl TimestampMs {
    pub const ZERO: Self = Self(0);

    pub fn elapsed_since(self, earlier: Self) -> u64 {
        self.0.saturating_sub(earlier.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputState {
    Inactive,
    Active,
}

impl OutputState {
    pub fn as_bool(self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSnapshot {
    pub bits: u32,
    pub timestamp: TimestampMs,
}

impl InputSnapshot {
    pub const EMPTY: Self = Self {
        bits: 0,
        timestamp: TimestampMs::ZERO,
    };

    pub fn is_active(self, input: InputId) -> bool {
        let shift = u32::from(input.0);

        if shift >= u32::BITS {
            return false;
        }

        (self.bits & (1_u32 << shift)) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemperatureSample {
    pub milli_celsius: i32,
    pub timestamp: TimestampMs,
}

impl TemperatureSample {
    pub const INVALID: Self = Self {
        milli_celsius: 0,
        timestamp: TimestampMs::ZERO,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    Platform,
    InvalidConfiguration,
    Telemetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Booting,
    Running,
    Faulted(Fault),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MqttState {
    Disconnected,
    Connected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Boot { timestamp: TimestampMs },
    Tick { timestamp: TimestampMs },
    MqttConnected { timestamp: TimestampMs },
    MqttDisconnected { timestamp: TimestampMs },
    InputChanged { snapshot: InputSnapshot },
}
