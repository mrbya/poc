#![no_std]

use poc_adapter::{start_tick_runtime, Platform, ZephyrPlatform};
use poc_app::App;
use poc_core::{AppEvent, TimestampMs};

extern crate zephyr;

const OUTPUT_COUNT: usize = 4;

static mut APP: App<OUTPUT_COUNT> = App::new();
static mut PLATFORM: ZephyrPlatform = ZephyrPlatform::new();

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().ok();
    }

    dispatch(AppEvent::Boot {
        timestamp: TimestampMs(unsafe { PLATFORM.uptime_ms().0 }),
    });
    start_tick_runtime();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum PocEventKind {
    Boot = 1,
    Tick = 2,
    MqttConnected = 3,
    MqttDisconnected = 4,
    Fault = 5,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PocEvent {
    pub kind: PocEventKind,
    pub timestamp_ms: u64,
    pub arg0: u32,
    pub arg1: u32,
}

#[no_mangle]
extern "C" fn poc_dispatch_event(event: PocEvent) {
    let timestamp = TimestampMs(event.timestamp_ms);
    let app_event = match event.kind {
        PocEventKind::Boot => AppEvent::Boot { timestamp },
        PocEventKind::Tick => AppEvent::Tick { timestamp },
        PocEventKind::MqttConnected => AppEvent::MqttConnected { timestamp },
        PocEventKind::MqttDisconnected => AppEvent::MqttDisconnected { timestamp },
        PocEventKind::Fault => return,
    };

    dispatch(app_event);
}

fn dispatch(event: AppEvent) {
    unsafe {
        let result = APP.dispatch(&mut PLATFORM, event);

        if result.is_err() {
            let _ = PLATFORM.log_info("app dispatch failed");
        }
    }
}
