#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

typedef enum {
    Boot = 1,
    Tick = 2,
    MqttConnected = 3,
    MqttDisconnected = 4,
    Fault = 5,
} poc_event_kind_t;

typedef struct {
    poc_event_kind_t kind;
    uint64_t timestamp_ms;
    uint32_t arg0;
    uint32_t arg1;
} poc_event_t;

void poc_dispatch_event(poc_event_t event);

#ifdef __cplusplus
}
#endif // __cplusplus
