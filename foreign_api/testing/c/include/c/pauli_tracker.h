#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A container of multiple Pauli frames, using a generic `Storage` type (that
 * implements [PauliStorage] if it shall be useful) as internal storage. The type
 * implements the core functionality to track the Pauli frames through a Clifford
 * circuit. As example view the documentation of [Circuit](crate::circuit::Circuit).
 * The explicit storage type should have the [PauliVec]s on it's minor axis (this is
 * more or less enforced by [PauliStorage]). The module [storage] provides some
 * compatible storage types.
 */
typedef struct Frames_MappedVector Frames_MappedVector;

typedef struct MappedVector MappedVector;

/**
 * Multiple encoded Paulis compressed into two [BitVec]s.
 */
typedef struct PauliVec PauliVec;

typedef struct MappedVector Storage;

typedef struct Frames_MappedVector Tracker;

typedef struct RawVec_PauliVec {
  struct PauliVec *ptr;
  uintptr_t len;
  uintptr_t cap;
} RawVec_PauliVec;

typedef struct RawVec_usize {
  uintptr_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} RawVec_usize;

typedef struct RawStorage {
  struct RawVec_PauliVec frames;
  struct RawVec_usize inverse_position;
} RawStorage;

typedef struct Tuple {
  uintptr_t qubit;
  const struct PauliVec *pauli;
} Tuple;

typedef struct RawVec_Tuple {
  struct Tuple *ptr;
  uintptr_t len;
  uintptr_t cap;
} RawVec_Tuple;

typedef struct RawVec_u32 {
  uint32_t *ptr;
  uintptr_t len;
  uintptr_t cap;
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
void free_storage(Storage *storage);

Tracker *new_tracker(void);

/**
 * # Safety
 */
void free_tracker(Tracker *tracker);

Storage *tracker_storage(const Tracker *tracker);

struct RawStorage raw_storage(Storage *storage);

void track_x(Tracker *tracker, uintptr_t qubit);

void track_z(Tracker *tracker, uintptr_t qubit);

void track_y(Tracker *tracker, uintptr_t qubit);

void apply_h(Tracker *tracker, uintptr_t qubit);

void apply_s(Tracker *tracker, uintptr_t qubit);

void apply_cx(Tracker *tracker, uintptr_t control, uintptr_t target);

void apply_cz(Tracker *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void measure_and_store(Tracker *tracker, uintptr_t qubit, Storage *storage);

void new_qubit(Tracker *tracker, uintptr_t qubit);

struct RawVec_Tuple sort_storage(const Storage *storage);

void free_sorted_storage(struct RawVec_Tuple raw_vec);

struct RawPauliVec raw_pauli_vec(struct PauliVec *pauli_vec);
