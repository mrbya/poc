#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    Ok = 0,
    Io = -1,
    InvalidArg = -2,
    Timeout = -3,
    Busy = -4,
    Internal = -5
} poc_status_t;

typedef struct {
    const uint8_t *ptr;
    size_t len;
} poc_bytes_t;

uint64_t poc_uptime_ms(void);
poc_status_t poc_log_info(poc_bytes_t message); 
void poc_platform_start_tick_runtime(void);

#ifdef __cplusplus
}
#endif // __cplusplus
