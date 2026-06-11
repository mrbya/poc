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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputSnapshot {
    pub bits: u32,
    pub timestamp: TimestampMs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemperatureSample {
    pub milli_celsius: i32,
    pub timestamp: TimestampMs,
}
