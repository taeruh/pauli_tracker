#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct MappedVector MappedVector;

/**
 * Multiple encoded Paulis compressed into two [BitVec]s.
 */
typedef struct PauliVec PauliVec;

typedef struct MappedVector Storage;

typedef struct RawVec_PauliVec {
  struct PauliVec *ptr;
  uintptr_t len;
} RawVec_PauliVec;

typedef struct RawVec_usize {
  uintptr_t *ptr;
  uintptr_t len;
} RawVec_usize;

typedef struct RawStorage {
  struct RawVec_PauliVec frames;
  struct RawVec_usize inverse_position;
} RawStorage;

typedef struct RawVec_u32 {
  uint32_t *ptr;
  uintptr_t len;
} RawVec_u32;

typedef struct RawPauliVec {
  struct RawVec_u32 left;
  uintptr_t left_len;
  struct RawVec_u32 right;
  uintptr_t right_len;
} RawPauliVec;

Storage *new_storage(void);

/**
 * # Safety
 */
struct RawStorage raw_storage(Storage *storage);

/**
 * # Safety
 */
void test(Storage *storage);

/**
 * # Safety
 */
struct RawPauliVec raw_pauli_vec(struct PauliVec *pauli_vec);
