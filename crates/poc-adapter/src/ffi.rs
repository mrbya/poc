use poc_core::{Error, Result, TimestampMs};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PocBytes {
    pub ptr: *const u8,
    pub len: usize,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PocStatus {
    Ok = 0,
    Io = -1,
    InvalidArgument = -2,
    Timeout = -3,
    Busy = -4,
    Internal = -5,
}

impl PocStatus {
    pub fn as_result(self) -> Result<()> {
        match self {
            Self::Ok => Ok(()),
            Self::Io => Err(Error::Io),
            Self::InvalidArgument => Err(Error::InvalidArgument),
            Self::Timeout => Err(Error::InvalidArgument),
            Self::Busy => Err(Error::Busy),
            Self::Internal => Err(Error::Internal),
        }
    }
}

unsafe extern "C" {
    fn poc_uptime_ms() -> u64;
    fn poc_log_info(message: PocBytes) -> PocStatus;
}

pub fn uptime_ms() -> TimestampMs {
    TimestampMs(unsafe { poc_uptime_ms() })
}

pub fn log_info(message: &str) -> Result<()> {
    let message = PocBytes {
        ptr: message.as_ptr(),
        len: message.len(),
    };

    let status = unsafe { poc_log_info(message) };
    status.as_result()
}
