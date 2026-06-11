#include <abi/abi.h>

#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(poc_abi, LOG_LEVEL_INF);

extern "C" uint64_t poc_uptime_ms() {
    return static_cast<uint64_t>(k_uptime_get());
}

extern "C" poc_status_t poc_log_info(poc_bytes_t message) {
    if ((message.ptr == nullptr) && (message.len != 0U)) {
        return poc_status_t(InvalidArg);
    }

    LOG_INF("%.*s",
            static_cast<int>(message.len),
            reinterpret_cast<const char*>(message.ptr)
    );

    return poc_status_t(Ok);
}
