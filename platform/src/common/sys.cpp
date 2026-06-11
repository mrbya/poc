#include <api.h>
#include "zephyr/toolchain.h"

#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(poc_sys, LOG_LEVEL_INF);

namespace {

constexpr auto tick_period = K_SECONDS(1);

void tick_work_handler(k_work* work);

K_WORK_DEFINE(tick_work, tick_work_handler);

void tick_timer_handler(k_timer* timer) {
    ARG_UNUSED(timer);

    k_work_submit(&tick_work);
}

K_TIMER_DEFINE(tick_timer, tick_timer_handler, nullptr);

void tick_work_handler(k_work* work) {
    ARG_UNUSED(work);

    const poc_event_t event {
        .kind = poc_event_kind_t(Tick),
        .timestamp_ms = static_cast<uint64_t>(k_uptime_get()),
        .arg0 = 0u,
        .arg1 = 0u,
    };

    poc_dispatch_event(event);
}

} // namespace

extern "C" void poc_start_tick_runtime(void) {
    LOG_INF("starting PoC tick runtime");
    k_timer_start(&tick_timer, tick_period, tick_period);
}
