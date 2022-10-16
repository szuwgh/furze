#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define BIT_FINAL_STATE (1 << 0)

#define BIT_LAST_STATE (1 << 1)

#define BIT_TAGET_NEXT (1 << 2)

#define BIT_STOP_NODE (1 << 3)

#define BIT_STATE_HAS_OUPPUT (1 << 4)

#define BIT_STATE_HAS_FINAL_OUTPUT (1 << 5)

#define BIT_TARGET_DELTA (1 << 6)

void *new_fst_builder(void);

int32_t add_key(void *arg, const uint8_t *key, uint32_t len, uint64_t value);

int32_t finish(void *arg);

const uint8_t *bytes(void *arg, uint32_t *len, uint32_t *cap);

void *load(uint8_t *key, uint32_t len, uint32_t cap);

int64_t find(void *arg, const uint8_t *key, uint32_t len);

int64_t get_first_key(void *arg, const uint8_t *key, uint32_t len);
