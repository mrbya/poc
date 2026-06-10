#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    ok = 0,
    io = -1,
    invalid_arg = -2,
    timeout = -3,
    busy = -4,
    internal = -5
} poc_status_t;

typedef struct {
    const uint8_t *ptr;
    size_t len;
} poc_bytes_t;

uint64_t poc_uptime_ms(void);
poc_status_t poc_log_info(poc_bytes_t message); 

#ifdef __cplusplus
}
#endif // __cplusplus
