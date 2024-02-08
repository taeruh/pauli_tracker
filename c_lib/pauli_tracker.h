#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Pauli described as enum.
 *
 * The discrimants are set according to [tableau_encoding]. Internally, it is very much
 * like [PauliDense](super::dense::PauliDense) (cf. [module](super)).
 */
enum PauliEnum
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Identity
   */
  I = 0,
  /**
   * Pauli Z
   */
  Z = 1,
  /**
   * Pauli X
   */
  X = 2,
  /**
   * Pauli Y
   */
  Y = 3,
};
#ifndef __cplusplus
typedef uint8_t PauliEnum;
#endif // __cplusplus

typedef struct BitVec BitVec;

/**
 * A newtype wrapper around [Vec], implementing the [collection](super) traits.
 *
 * Since we cannot arbitrarily insert and remove elements, inserting as only allowed
 * for keys bigger than the current length of the vector (inserting additional buffer
 * elements if necessary), and only the last element can be removed.
 */
typedef struct BufferedVector_PauliEnum BufferedVector_PauliEnum;

/**
 * A newtype wrapper around [Vec], implementing the [collection](super) traits.
 *
 * Since we cannot arbitrarily insert and remove elements, inserting as only allowed
 * for keys bigger than the current length of the vector (inserting additional buffer
 * elements if necessary), and only the last element can be removed.
 */
typedef struct BufferedVector_PauliStack_bv BufferedVector_PauliStack_bv;

/**
 * A newtype wrapper around [Vec], implementing the [collection](super) traits.
 *
 * Since we cannot arbitrarily insert and remove elements, inserting as only allowed
 * for keys bigger than the current length of the vector (inserting additional buffer
 * elements if necessary), and only the last element can be removed.
 */
typedef struct BufferedVector_PauliStack_vb BufferedVector_PauliStack_vb;

/**
 * A newtype wrapper around [Vec], implementing the [collection](super) traits.
 *
 * Since we cannot arbitrarily insert and remove elements, inserting as only allowed
 * for keys bigger than the current length of the vector (inserting additional buffer
 * elements if necessary), and only the last element can be removed.
 */
typedef struct BufferedVector_PauliTuple BufferedVector_PauliTuple;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_BufferedVector_psbv Frames_BufferedVector_psbv;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_BufferedVector_psvb Frames_BufferedVector_psvb;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_Map_psbvfx Frames_Map_psbvfx;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_Map_psvbfx Frames_Map_psvbfx;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_MappedVector_psbvfx Frames_MappedVector_psbvfx;

/**
 * A container of multiple Pauli frames that implements [Tracker].
 *
 * Compare the [module documentation](super::frames). To be useful, the generic
 * `Storage` type should implement [IterableBase] (or better [Full]). The explicit
 * storage type should have the [PauliStack]s on it's minor axis (this is more or less
 * enforced by the [collection] traits). The [collection] module provides some
 * compatible storage types.
 *
 * [collection]: crate::collection
 */
typedef struct Frames_MappedVector_psvbfx Frames_MappedVector_psvbfx;

typedef struct HashMap_usize__PauliEnum__BuildHasherDefault_FxHasher HashMap_usize__PauliEnum__BuildHasherDefault_FxHasher;

typedef struct HashMap_usize__PauliStack_bv__BuildHasherDefault_FxHasher HashMap_usize__PauliStack_bv__BuildHasherDefault_FxHasher;

typedef struct HashMap_usize__PauliStack_vb__BuildHasherDefault_FxHasher HashMap_usize__PauliStack_vb__BuildHasherDefault_FxHasher;

typedef struct HashMap_usize__PauliTuple__BuildHasherDefault_FxHasher HashMap_usize__PauliTuple__BuildHasherDefault_FxHasher;

/**
 * An implementor of [Tracker] that tracks Pauli gates at runtime.
 *
 * Compare the [module documentation](super::live). To be useful, the generic `Storage`
 * type should at least implement [Base], with implementors of [Pauli] as elements.
 */
typedef struct Live_BufferedVector_pe Live_BufferedVector_pe;

/**
 * An implementor of [Tracker] that tracks Pauli gates at runtime.
 *
 * Compare the [module documentation](super::live). To be useful, the generic `Storage`
 * type should at least implement [Base], with implementors of [Pauli] as elements.
 */
typedef struct Live_BufferedVector_pt Live_BufferedVector_pt;

/**
 * An implementor of [Tracker] that tracks Pauli gates at runtime.
 *
 * Compare the [module documentation](super::live). To be useful, the generic `Storage`
 * type should at least implement [Base], with implementors of [Pauli] as elements.
 */
typedef struct Live_Map_pefx Live_Map_pefx;

/**
 * A mixture of a [Vec] and a [HashMap].
 *
 * The elements are stored in a [Vec] storage while accessing them is done through a
 * [HashMap] to get the right index in the storage. Inserting elements is done by
 * pushing to the storage and removing is done via swap-removes.
 *
 * [HashMap]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#
 * instead of going through _MappedVector we should implement it directly, at least for
 * the serialization, because we are unnecessarily cloning it there
 */
typedef struct MappedVector_PauliEnum__BuildHasherDefault_FxHasher MappedVector_PauliEnum__BuildHasherDefault_FxHasher;

/**
 * A mixture of a [Vec] and a [HashMap].
 *
 * The elements are stored in a [Vec] storage while accessing them is done through a
 * [HashMap] to get the right index in the storage. Inserting elements is done by
 * pushing to the storage and removing is done via swap-removes.
 *
 * [HashMap]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#
 * instead of going through _MappedVector we should implement it directly, at least for
 * the serialization, because we are unnecessarily cloning it there
 */
typedef struct MappedVector_PauliStack_bv__BuildHasherDefault_FxHasher MappedVector_PauliStack_bv__BuildHasherDefault_FxHasher;

/**
 * A mixture of a [Vec] and a [HashMap].
 *
 * The elements are stored in a [Vec] storage while accessing them is done through a
 * [HashMap] to get the right index in the storage. Inserting elements is done by
 * pushing to the storage and removing is done via swap-removes.
 *
 * [HashMap]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#
 * instead of going through _MappedVector we should implement it directly, at least for
 * the serialization, because we are unnecessarily cloning it there
 */
typedef struct MappedVector_PauliStack_vb__BuildHasherDefault_FxHasher MappedVector_PauliStack_vb__BuildHasherDefault_FxHasher;

/**
 * A mixture of a [Vec] and a [HashMap].
 *
 * The elements are stored in a [Vec] storage while accessing them is done through a
 * [HashMap] to get the right index in the storage. Inserting elements is done by
 * pushing to the storage and removing is done via swap-removes.
 *
 * [HashMap]: https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#
 * instead of going through _MappedVector we should implement it directly, at least for
 * the serialization, because we are unnecessarily cloning it there
 */
typedef struct MappedVector_PauliTuple__BuildHasherDefault_FxHasher MappedVector_PauliTuple__BuildHasherDefault_FxHasher;

/**
 * Multiple encoded Paulis compressed into two [BooleanVector]s.
 *
 * Instead of having a vector over [Pauli]s, we separate the X and Z parts into two
 * vectors (cf. [Pauli] for encoding). This enables us to efficiently perform
 * (Clifford) operations on those [PauliStack]s.
 *
 * Note that the fields are public and the methods are mainly convenience methods.
 */
typedef struct PauliStack_BitVec PauliStack_BitVec;

/**
 * Multiple encoded Paulis compressed into two [BooleanVector]s.
 *
 * Instead of having a vector over [Pauli]s, we separate the X and Z parts into two
 * vectors (cf. [Pauli] for encoding). This enables us to efficiently perform
 * (Clifford) operations on those [PauliStack]s.
 *
 * Note that the fields are public and the methods are mainly convenience methods.
 */
typedef struct PauliStack_Vec_b PauliStack_Vec_b;

typedef struct Vec_bool Vec_bool;

typedef struct Vec_bool Vec_b;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_bool {
  /**
   * Pointer to the start of the vector.
   */
  bool *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_bool;

typedef struct RawVec_bool RawVec_b;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_u64 {
  /**
   * Pointer to the start of the vector.
   */
  uint64_t *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_u64;

/**
 * A [HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#) with
 * [usize] keys.
 */
typedef struct HashMap_usize__PauliStack_vb__BuildHasherDefault_FxHasher Map_PauliStack_vb__BuildHasherDefault_FxHasher;

typedef Map_PauliStack_vb__BuildHasherDefault_FxHasher Map_psvbfx;

typedef struct PauliStack_Vec_b PauliStack_vb;

/**
 * A [HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#) with
 * [usize] keys.
 */
typedef struct HashMap_usize__PauliStack_bv__BuildHasherDefault_FxHasher Map_PauliStack_bv__BuildHasherDefault_FxHasher;

typedef Map_PauliStack_bv__BuildHasherDefault_FxHasher Map_psbvfx;

typedef struct PauliStack_BitVec PauliStack_bv;

/**
 * A [HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#) with
 * [usize] keys.
 */
typedef struct HashMap_usize__PauliEnum__BuildHasherDefault_FxHasher Map_PauliEnum__BuildHasherDefault_FxHasher;

typedef Map_PauliEnum__BuildHasherDefault_FxHasher Map_pefx;

/**
 * A [HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html#) with
 * [usize] keys.
 */
typedef struct HashMap_usize__PauliTuple__BuildHasherDefault_FxHasher Map_PauliTuple__BuildHasherDefault_FxHasher;

typedef Map_PauliTuple__BuildHasherDefault_FxHasher Map_ptfx;

/**
 * A Pauli represented by two booleans values. The first one is the X part and the
 * second one is the Z part.
 */
typedef struct PauliTuple {
  /**
   * X part
   */
  bool _0;
  /**
   * Z part
   */
  bool _1;
} PauliTuple;

typedef struct MappedVector_PauliStack_vb__BuildHasherDefault_FxHasher MappedVector_psvbfx;

typedef struct MappedVector_PauliStack_bv__BuildHasherDefault_FxHasher MappedVector_psbvfx;

typedef struct MappedVector_PauliEnum__BuildHasherDefault_FxHasher MappedVector_pefx;

typedef struct MappedVector_PauliTuple__BuildHasherDefault_FxHasher MappedVector_ptfx;

typedef struct BufferedVector_PauliStack_vb BufferedVector_psvb;

typedef struct BufferedVector_PauliStack_bv BufferedVector_psbv;

typedef struct BufferedVector_PauliEnum BufferedVector_pe;

typedef struct BufferedVector_PauliTuple BufferedVector_pt;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_PauliStack_vb {
  /**
   * Pointer to the start of the vector.
   */
  PauliStack_vb *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_PauliStack_vb;

typedef struct RawVec_PauliStack_vb RawVec_psvb;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_PauliStack_bv {
  /**
   * Pointer to the start of the vector.
   */
  PauliStack_bv *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_PauliStack_bv;

typedef struct RawVec_PauliStack_bv RawVec_psbv;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_PauliEnum {
  /**
   * Pointer to the start of the vector.
   */
  PauliEnum *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_PauliEnum;

typedef struct RawVec_PauliEnum RawVec_pe;

/**
 * This struct can be used to build a native `Vec<T>` for easier ergonomics, however,
 * keep the [safety considerations](self#safety-considerations) in mind.
 */
typedef struct RawVec_PauliTuple {
  /**
   * Pointer to the start of the vector.
   */
  struct PauliTuple *data;
  /**
   * The length of the vector.
   */
  uintptr_t len;
} RawVec_PauliTuple;

typedef struct RawVec_PauliTuple RawVec_pt;

typedef struct Frames_Map_psbvfx Frames_hmpsbvfx;

typedef struct Frames_Map_psvbfx Frames_hmpsvbfx;

typedef struct Frames_BufferedVector_psvb Frames_bvpsvb;

typedef struct Frames_BufferedVector_psbv Frames_bvpsbv;

typedef struct Frames_MappedVector_psvbfx Frames_mvpsvbfx;

typedef struct Frames_MappedVector_psbvfx Frames_mvpsbvfx;

typedef struct Live_Map_pefx Live_hmpefx;

typedef struct Live_BufferedVector_pe Live_bvpe;

typedef struct Live_BufferedVector_pt Live_bvpt;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Vec_b *vec_b_new(void);

/**
 * Frees the input instance.
 */
void vec_b_free(Vec_b *x);

/**
 * Serialize into json.
 */
void vec_b_serialize(const Vec_b *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Vec_b *vec_b_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void vec_b_serialize_bin(const Vec_b *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Vec_b *vec_b_deserialize_bin(const char *file);

bool vec_b_get(Vec_b *x, uintptr_t key);

uintptr_t vec_b_len(const Vec_b *x);

bool vec_b_is_empty(const Vec_b *x);

void vec_b_resize(Vec_b *x, uintptr_t len, bool flag);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
struct BitVec *bitvec_new(void);

/**
 * Frees the input instance.
 */
void bitvec_free(struct BitVec *x);

/**
 * Serialize into json.
 */
void bitvec_serialize(const struct BitVec *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
struct BitVec *bitvec_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void bitvec_serialize_bin(const struct BitVec *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
struct BitVec *bitvec_deserialize_bin(const char *file);

bool bitvec_get(struct BitVec *x, uintptr_t key);

uintptr_t bitvec_len(const struct BitVec *x);

bool bitvec_is_empty(const struct BitVec *x);

void bitvec_resize(struct BitVec *x, uintptr_t len, bool flag);

RawVec_b vec_b_get_raw(Vec_b *x);

/**
 * Note that the `len`gth is not the number of bits, but the number of storage chunks.
 */
struct RawVec_u64 bitvec_get_raw(struct BitVec *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psvbfx *map_psvbfx_new(void);

/**
 * Frees the input instance.
 */
void map_psvbfx_free(Map_psvbfx *x);

/**
 * Serialize into json.
 */
void map_psvbfx_serialize(const Map_psvbfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psvbfx *map_psvbfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void map_psvbfx_serialize_bin(const Map_psvbfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psvbfx *map_psvbfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psvbfx *map_psvbfx_init(uintptr_t num_qubits);

const PauliStack_vb *map_psvbfx_get(const Map_psvbfx *x, uintptr_t key);

PauliStack_vb *map_psvbfx_get_mut(Map_psvbfx *x, uintptr_t key);

uintptr_t map_psvbfx_len(const Map_psvbfx *x);

bool map_psvbfx_is_empty(const Map_psvbfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psbvfx *map_psbvfx_new(void);

/**
 * Frees the input instance.
 */
void map_psbvfx_free(Map_psbvfx *x);

/**
 * Serialize into json.
 */
void map_psbvfx_serialize(const Map_psbvfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psbvfx *map_psbvfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void map_psbvfx_serialize_bin(const Map_psbvfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psbvfx *map_psbvfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psbvfx *map_psbvfx_init(uintptr_t num_qubits);

const PauliStack_bv *map_psbvfx_get(const Map_psbvfx *x, uintptr_t key);

PauliStack_bv *map_psbvfx_get_mut(Map_psbvfx *x, uintptr_t key);

uintptr_t map_psbvfx_len(const Map_psbvfx *x);

bool map_psbvfx_is_empty(const Map_psbvfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_pefx *map_pefx_new(void);

/**
 * Frees the input instance.
 */
void map_pefx_free(Map_pefx *x);

/**
 * Serialize into json.
 */
void map_pefx_serialize(const Map_pefx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_pefx *map_pefx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void map_pefx_serialize_bin(const Map_pefx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_pefx *map_pefx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_pefx *map_pefx_init(uintptr_t num_qubits);

const PauliEnum *map_pefx_get(const Map_pefx *x, uintptr_t key);

PauliEnum *map_pefx_get_mut(Map_pefx *x, uintptr_t key);

uintptr_t map_pefx_len(const Map_pefx *x);

bool map_pefx_is_empty(const Map_pefx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_ptfx *map_ptfx_new(void);

/**
 * Frees the input instance.
 */
void map_ptfx_free(Map_ptfx *x);

/**
 * Serialize into json.
 */
void map_ptfx_serialize(const Map_ptfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_ptfx *map_ptfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void map_ptfx_serialize_bin(const Map_ptfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_ptfx *map_ptfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_ptfx *map_ptfx_init(uintptr_t num_qubits);

const struct PauliTuple *map_ptfx_get(const Map_ptfx *x, uintptr_t key);

struct PauliTuple *map_ptfx_get_mut(Map_ptfx *x, uintptr_t key);

uintptr_t map_ptfx_len(const Map_ptfx *x);

bool map_ptfx_is_empty(const Map_ptfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psvbfx *mapped_vector_psvbfx_new(void);

/**
 * Frees the input instance.
 */
void mapped_vector_psvbfx_free(MappedVector_psvbfx *x);

/**
 * Serialize into json.
 */
void mapped_vector_psvbfx_serialize(const MappedVector_psvbfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psvbfx *mapped_vector_psvbfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void mapped_vector_psvbfx_serialize_bin(const MappedVector_psvbfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psvbfx *mapped_vector_psvbfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psvbfx *mapped_vector_psvbfx_init(uintptr_t num_qubits);

const PauliStack_vb *mapped_vector_psvbfx_get(const MappedVector_psvbfx *x, uintptr_t key);

PauliStack_vb *mapped_vector_psvbfx_get_mut(MappedVector_psvbfx *x, uintptr_t key);

uintptr_t mapped_vector_psvbfx_len(const MappedVector_psvbfx *x);

bool mapped_vector_psvbfx_is_empty(const MappedVector_psvbfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psbvfx *mapped_vector_psbvfx_new(void);

/**
 * Frees the input instance.
 */
void mapped_vector_psbvfx_free(MappedVector_psbvfx *x);

/**
 * Serialize into json.
 */
void mapped_vector_psbvfx_serialize(const MappedVector_psbvfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psbvfx *mapped_vector_psbvfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void mapped_vector_psbvfx_serialize_bin(const MappedVector_psbvfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psbvfx *mapped_vector_psbvfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psbvfx *mapped_vector_psbvfx_init(uintptr_t num_qubits);

const PauliStack_bv *mapped_vector_psbvfx_get(const MappedVector_psbvfx *x, uintptr_t key);

PauliStack_bv *mapped_vector_psbvfx_get_mut(MappedVector_psbvfx *x, uintptr_t key);

uintptr_t mapped_vector_psbvfx_len(const MappedVector_psbvfx *x);

bool mapped_vector_psbvfx_is_empty(const MappedVector_psbvfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_pefx *mapped_vector_pefx_new(void);

/**
 * Frees the input instance.
 */
void mapped_vector_pefx_free(MappedVector_pefx *x);

/**
 * Serialize into json.
 */
void mapped_vector_pefx_serialize(const MappedVector_pefx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_pefx *mapped_vector_pefx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void mapped_vector_pefx_serialize_bin(const MappedVector_pefx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_pefx *mapped_vector_pefx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_pefx *mapped_vector_pefx_init(uintptr_t num_qubits);

const PauliEnum *mapped_vector_pefx_get(const MappedVector_pefx *x, uintptr_t key);

PauliEnum *mapped_vector_pefx_get_mut(MappedVector_pefx *x, uintptr_t key);

uintptr_t mapped_vector_pefx_len(const MappedVector_pefx *x);

bool mapped_vector_pefx_is_empty(const MappedVector_pefx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_ptfx *mapped_vector_ptfx_new(void);

/**
 * Frees the input instance.
 */
void mapped_vector_ptfx_free(MappedVector_ptfx *x);

/**
 * Serialize into json.
 */
void mapped_vector_ptfx_serialize(const MappedVector_ptfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_ptfx *mapped_vector_ptfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void mapped_vector_ptfx_serialize_bin(const MappedVector_ptfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_ptfx *mapped_vector_ptfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_ptfx *mapped_vector_ptfx_init(uintptr_t num_qubits);

const struct PauliTuple *mapped_vector_ptfx_get(const MappedVector_ptfx *x, uintptr_t key);

struct PauliTuple *mapped_vector_ptfx_get_mut(MappedVector_ptfx *x, uintptr_t key);

uintptr_t mapped_vector_ptfx_len(const MappedVector_ptfx *x);

bool mapped_vector_ptfx_is_empty(const MappedVector_ptfx *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *buffered_vector_psvb_new(void);

/**
 * Frees the input instance.
 */
void buffered_vector_psvb_free(BufferedVector_psvb *x);

/**
 * Serialize into json.
 */
void buffered_vector_psvb_serialize(const BufferedVector_psvb *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *buffered_vector_psvb_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void buffered_vector_psvb_serialize_bin(const BufferedVector_psvb *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *buffered_vector_psvb_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *buffered_vector_psvb_init(uintptr_t num_qubits);

const PauliStack_vb *buffered_vector_psvb_get(const BufferedVector_psvb *x, uintptr_t key);

PauliStack_vb *buffered_vector_psvb_get_mut(BufferedVector_psvb *x, uintptr_t key);

uintptr_t buffered_vector_psvb_len(const BufferedVector_psvb *x);

bool buffered_vector_psvb_is_empty(const BufferedVector_psvb *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *buffered_vector_psbv_new(void);

/**
 * Frees the input instance.
 */
void buffered_vector_psbv_free(BufferedVector_psbv *x);

/**
 * Serialize into json.
 */
void buffered_vector_psbv_serialize(const BufferedVector_psbv *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *buffered_vector_psbv_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void buffered_vector_psbv_serialize_bin(const BufferedVector_psbv *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *buffered_vector_psbv_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *buffered_vector_psbv_init(uintptr_t num_qubits);

const PauliStack_bv *buffered_vector_psbv_get(const BufferedVector_psbv *x, uintptr_t key);

PauliStack_bv *buffered_vector_psbv_get_mut(BufferedVector_psbv *x, uintptr_t key);

uintptr_t buffered_vector_psbv_len(const BufferedVector_psbv *x);

bool buffered_vector_psbv_is_empty(const BufferedVector_psbv *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pe *buffered_vector_pe_new(void);

/**
 * Frees the input instance.
 */
void buffered_vector_pe_free(BufferedVector_pe *x);

/**
 * Serialize into json.
 */
void buffered_vector_pe_serialize(const BufferedVector_pe *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pe *buffered_vector_pe_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void buffered_vector_pe_serialize_bin(const BufferedVector_pe *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pe *buffered_vector_pe_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pe *buffered_vector_pe_init(uintptr_t num_qubits);

const PauliEnum *buffered_vector_pe_get(const BufferedVector_pe *x, uintptr_t key);

PauliEnum *buffered_vector_pe_get_mut(BufferedVector_pe *x, uintptr_t key);

uintptr_t buffered_vector_pe_len(const BufferedVector_pe *x);

bool buffered_vector_pe_is_empty(const BufferedVector_pe *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pt *buffered_vector_pt_new(void);

/**
 * Frees the input instance.
 */
void buffered_vector_pt_free(BufferedVector_pt *x);

/**
 * Serialize into json.
 */
void buffered_vector_pt_serialize(const BufferedVector_pt *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pt *buffered_vector_pt_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void buffered_vector_pt_serialize_bin(const BufferedVector_pt *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pt *buffered_vector_pt_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pt *buffered_vector_pt_init(uintptr_t num_qubits);

const struct PauliTuple *buffered_vector_pt_get(const BufferedVector_pt *x, uintptr_t key);

struct PauliTuple *buffered_vector_pt_get_mut(BufferedVector_pt *x, uintptr_t key);

uintptr_t buffered_vector_pt_len(const BufferedVector_pt *x);

bool buffered_vector_pt_is_empty(const BufferedVector_pt *x);

RawVec_psvb buffered_vector_psvb_get_raw(BufferedVector_psvb *x);

RawVec_psbv buffered_vector_psbv_get_raw(BufferedVector_psbv *x);

RawVec_pe buffered_vector_pe_get_raw(BufferedVector_pe *x);

RawVec_pt buffered_vector_pt_get_raw(BufferedVector_pt *x);

uint8_t pauli_enum_tableau_encoding(PauliEnum *x);

uint8_t pauli_tuple_tableau_encoding(struct PauliTuple *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *pauli_stack_vb_new(void);

/**
 * Frees the input instance.
 */
void pauli_stack_vb_free(PauliStack_vb *x);

/**
 * Serialize into json.
 */
void pauli_stack_vb_serialize(const PauliStack_vb *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *pauli_stack_vb_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void pauli_stack_vb_serialize_bin(const PauliStack_vb *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *pauli_stack_vb_deserialize_bin(const char *file);

Vec_b *pauli_stack_vb_x(PauliStack_vb *x);

Vec_b *pauli_stack_vb_z(PauliStack_vb *x);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *pauli_stack_bv_new(void);

/**
 * Frees the input instance.
 */
void pauli_stack_bv_free(PauliStack_bv *x);

/**
 * Serialize into json.
 */
void pauli_stack_bv_serialize(const PauliStack_bv *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *pauli_stack_bv_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void pauli_stack_bv_serialize_bin(const PauliStack_bv *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *pauli_stack_bv_deserialize_bin(const char *file);

struct BitVec *pauli_stack_bv_x(PauliStack_bv *x);

struct BitVec *pauli_stack_bv_z(PauliStack_bv *x);

/**
 * don't use this
 */
void show_frames(const Frames_hmpsbvfx *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsvbfx *frames_hmpsvbfx_new(void);

/**
 * Frees the input instance.
 */
void frames_hmpsvbfx_free(Frames_hmpsvbfx *x);

/**
 * Serialize into json.
 */
void frames_hmpsvbfx_serialize(const Frames_hmpsvbfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsvbfx *frames_hmpsvbfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_hmpsvbfx_serialize_bin(const Frames_hmpsvbfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsvbfx *frames_hmpsvbfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsvbfx *frames_hmpsvbfx_init(uintptr_t num_qubits);

void frames_hmpsvbfx_track_x(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_track_y(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_track_z(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_id(Frames_hmpsvbfx*, uintptr_t);

void frames_hmpsvbfx_x(Frames_hmpsvbfx*, uintptr_t);

void frames_hmpsvbfx_y(Frames_hmpsvbfx*, uintptr_t);

void frames_hmpsvbfx_z(Frames_hmpsvbfx*, uintptr_t);

void frames_hmpsvbfx_s(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sdg(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sz(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_szdg(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_hxy(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_h(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sy(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sydg(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sh(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_hs(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_shs(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sx(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_sxdg(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_hyz(Frames_hmpsvbfx *tracker, uintptr_t qubit);

void frames_hmpsvbfx_cz(Frames_hmpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsvbfx_cx(Frames_hmpsvbfx *tracker, uintptr_t control, uintptr_t target);

void frames_hmpsvbfx_cy(Frames_hmpsvbfx *tracker, uintptr_t control, uintptr_t target);

void frames_hmpsvbfx_swap(Frames_hmpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsvbfx_iswap(Frames_hmpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsvbfx_iswapdg(Frames_hmpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsvbfx_move_x_to_x(Frames_hmpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsvbfx_move_x_to_z(Frames_hmpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsvbfx_move_z_to_x(Frames_hmpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsvbfx_move_z_to_z(Frames_hmpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsvbfx_new_qubit(Frames_hmpsvbfx *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *frames_hmpsvbfx_measure(Frames_hmpsvbfx *tracker,
                                       uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *frames_hmpsvbfx_stacked_transpose(Frames_hmpsvbfx *frames,
                                                       uintptr_t num_frames);

uintptr_t frames_hmpsvbfx_frames_num(Frames_hmpsvbfx *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsvbfx *frames_hmpsvbfx_new_unchecked(Map_psvbfx *storage,
                                               uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psvbfx *frames_hmpsvbfx_into_storage(Frames_hmpsvbfx *frames);

const Map_psvbfx *frames_hmpsvbfx_as_storage(Frames_hmpsvbfx *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsbvfx *frames_hmpsbvfx_new(void);

/**
 * Frees the input instance.
 */
void frames_hmpsbvfx_free(Frames_hmpsbvfx *x);

/**
 * Serialize into json.
 */
void frames_hmpsbvfx_serialize(const Frames_hmpsbvfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsbvfx *frames_hmpsbvfx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_hmpsbvfx_serialize_bin(const Frames_hmpsbvfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsbvfx *frames_hmpsbvfx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsbvfx *frames_hmpsbvfx_init(uintptr_t num_qubits);

void frames_hmpsbvfx_track_x(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_track_y(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_track_z(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_id(Frames_hmpsbvfx*, uintptr_t);

void frames_hmpsbvfx_x(Frames_hmpsbvfx*, uintptr_t);

void frames_hmpsbvfx_y(Frames_hmpsbvfx*, uintptr_t);

void frames_hmpsbvfx_z(Frames_hmpsbvfx*, uintptr_t);

void frames_hmpsbvfx_s(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sdg(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sz(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_szdg(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_hxy(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_h(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sy(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sydg(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sh(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_hs(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_shs(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sx(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_sxdg(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_hyz(Frames_hmpsbvfx *tracker, uintptr_t qubit);

void frames_hmpsbvfx_cz(Frames_hmpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsbvfx_cx(Frames_hmpsbvfx *tracker, uintptr_t control, uintptr_t target);

void frames_hmpsbvfx_cy(Frames_hmpsbvfx *tracker, uintptr_t control, uintptr_t target);

void frames_hmpsbvfx_swap(Frames_hmpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsbvfx_iswap(Frames_hmpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsbvfx_iswapdg(Frames_hmpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_hmpsbvfx_move_x_to_x(Frames_hmpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsbvfx_move_x_to_z(Frames_hmpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsbvfx_move_z_to_x(Frames_hmpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsbvfx_move_z_to_z(Frames_hmpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_hmpsbvfx_new_qubit(Frames_hmpsbvfx *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *frames_hmpsbvfx_measure(Frames_hmpsbvfx *tracker,
                                       uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *frames_hmpsbvfx_stacked_transpose(Frames_hmpsbvfx *frames,
                                                       uintptr_t num_frames);

uintptr_t frames_hmpsbvfx_frames_num(Frames_hmpsbvfx *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_hmpsbvfx *frames_hmpsbvfx_new_unchecked(Map_psbvfx *storage,
                                               uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_psbvfx *frames_hmpsbvfx_into_storage(Frames_hmpsbvfx *frames);

const Map_psbvfx *frames_hmpsbvfx_as_storage(Frames_hmpsbvfx *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsvb *frames_bvpsvb_new(void);

/**
 * Frees the input instance.
 */
void frames_bvpsvb_free(Frames_bvpsvb *x);

/**
 * Serialize into json.
 */
void frames_bvpsvb_serialize(const Frames_bvpsvb *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsvb *frames_bvpsvb_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_bvpsvb_serialize_bin(const Frames_bvpsvb *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsvb *frames_bvpsvb_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsvb *frames_bvpsvb_init(uintptr_t num_qubits);

void frames_bvpsvb_track_x(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_track_y(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_track_z(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_id(Frames_bvpsvb*, uintptr_t);

void frames_bvpsvb_x(Frames_bvpsvb*, uintptr_t);

void frames_bvpsvb_y(Frames_bvpsvb*, uintptr_t);

void frames_bvpsvb_z(Frames_bvpsvb*, uintptr_t);

void frames_bvpsvb_s(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sdg(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sz(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_szdg(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_hxy(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_h(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sy(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sydg(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sh(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_hs(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_shs(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sx(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_sxdg(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_hyz(Frames_bvpsvb *tracker, uintptr_t qubit);

void frames_bvpsvb_cz(Frames_bvpsvb *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsvb_cx(Frames_bvpsvb *tracker, uintptr_t control, uintptr_t target);

void frames_bvpsvb_cy(Frames_bvpsvb *tracker, uintptr_t control, uintptr_t target);

void frames_bvpsvb_swap(Frames_bvpsvb *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsvb_iswap(Frames_bvpsvb *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsvb_iswapdg(Frames_bvpsvb *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsvb_move_x_to_x(Frames_bvpsvb *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsvb_move_x_to_z(Frames_bvpsvb *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsvb_move_z_to_x(Frames_bvpsvb *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsvb_move_z_to_z(Frames_bvpsvb *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsvb_new_qubit(Frames_bvpsvb *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *frames_bvpsvb_measure(Frames_bvpsvb *tracker,
                                     uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *frames_bvpsvb_stacked_transpose(Frames_bvpsvb *frames,
                                                     uintptr_t num_frames);

uintptr_t frames_bvpsvb_frames_num(Frames_bvpsvb *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsvb *frames_bvpsvb_new_unchecked(BufferedVector_psvb *storage,
                                           uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *frames_bvpsvb_into_storage(Frames_bvpsvb *frames);

const BufferedVector_psvb *frames_bvpsvb_as_storage(Frames_bvpsvb *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsbv *frames_bvpsbv_new(void);

/**
 * Frees the input instance.
 */
void frames_bvpsbv_free(Frames_bvpsbv *x);

/**
 * Serialize into json.
 */
void frames_bvpsbv_serialize(const Frames_bvpsbv *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsbv *frames_bvpsbv_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_bvpsbv_serialize_bin(const Frames_bvpsbv *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsbv *frames_bvpsbv_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsbv *frames_bvpsbv_init(uintptr_t num_qubits);

void frames_bvpsbv_track_x(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_track_y(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_track_z(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_id(Frames_bvpsbv*, uintptr_t);

void frames_bvpsbv_x(Frames_bvpsbv*, uintptr_t);

void frames_bvpsbv_y(Frames_bvpsbv*, uintptr_t);

void frames_bvpsbv_z(Frames_bvpsbv*, uintptr_t);

void frames_bvpsbv_s(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sdg(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sz(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_szdg(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_hxy(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_h(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sy(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sydg(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sh(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_hs(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_shs(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sx(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_sxdg(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_hyz(Frames_bvpsbv *tracker, uintptr_t qubit);

void frames_bvpsbv_cz(Frames_bvpsbv *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsbv_cx(Frames_bvpsbv *tracker, uintptr_t control, uintptr_t target);

void frames_bvpsbv_cy(Frames_bvpsbv *tracker, uintptr_t control, uintptr_t target);

void frames_bvpsbv_swap(Frames_bvpsbv *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsbv_iswap(Frames_bvpsbv *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsbv_iswapdg(Frames_bvpsbv *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_bvpsbv_move_x_to_x(Frames_bvpsbv *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsbv_move_x_to_z(Frames_bvpsbv *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsbv_move_z_to_x(Frames_bvpsbv *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsbv_move_z_to_z(Frames_bvpsbv *tracker, uintptr_t source, uintptr_t destination);

void frames_bvpsbv_new_qubit(Frames_bvpsbv *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *frames_bvpsbv_measure(Frames_bvpsbv *tracker,
                                     uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *frames_bvpsbv_stacked_transpose(Frames_bvpsbv *frames,
                                                     uintptr_t num_frames);

uintptr_t frames_bvpsbv_frames_num(Frames_bvpsbv *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_bvpsbv *frames_bvpsbv_new_unchecked(BufferedVector_psbv *storage,
                                           uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *frames_bvpsbv_into_storage(Frames_bvpsbv *frames);

const BufferedVector_psbv *frames_bvpsbv_as_storage(Frames_bvpsbv *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsvbfx *frames_mvpsvb_new(void);

/**
 * Frees the input instance.
 */
void frames_mvpsvb_free(Frames_mvpsvbfx *x);

/**
 * Serialize into json.
 */
void frames_mvpsvb_serialize(const Frames_mvpsvbfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsvbfx *frames_mvpsvb_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_mvpsvb_serialize_bin(const Frames_mvpsvbfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsvbfx *frames_mvpsvb_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsvbfx *frames_mvpsvb_init(uintptr_t num_qubits);

void frames_mvpsvb_track_x(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_track_y(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_track_z(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_id(Frames_mvpsvbfx*, uintptr_t);

void frames_mvpsvb_x(Frames_mvpsvbfx*, uintptr_t);

void frames_mvpsvb_y(Frames_mvpsvbfx*, uintptr_t);

void frames_mvpsvb_z(Frames_mvpsvbfx*, uintptr_t);

void frames_mvpsvb_s(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sdg(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sz(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_szdg(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_hxy(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_h(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sy(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sydg(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sh(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_hs(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_shs(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sx(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_sxdg(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_hyz(Frames_mvpsvbfx *tracker, uintptr_t qubit);

void frames_mvpsvb_cz(Frames_mvpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsvb_cx(Frames_mvpsvbfx *tracker, uintptr_t control, uintptr_t target);

void frames_mvpsvb_cy(Frames_mvpsvbfx *tracker, uintptr_t control, uintptr_t target);

void frames_mvpsvb_swap(Frames_mvpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsvb_iswap(Frames_mvpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsvb_iswapdg(Frames_mvpsvbfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsvb_move_x_to_x(Frames_mvpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsvb_move_x_to_z(Frames_mvpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsvb_move_z_to_x(Frames_mvpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsvb_move_z_to_z(Frames_mvpsvbfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsvb_new_qubit(Frames_mvpsvbfx *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_vb *frames_mvpsvb_measure(Frames_mvpsvbfx *tracker,
                                     uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psvb *frames_mvpsvb_stacked_transpose(Frames_mvpsvbfx *frames,
                                                     uintptr_t num_frames);

uintptr_t frames_mvpsvb_frames_num(Frames_mvpsvbfx *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsvbfx *frames_mvpsvb_new_unchecked(MappedVector_psvbfx *storage,
                                             uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psvbfx *frames_mvpsvb_into_storage(Frames_mvpsvbfx *frames);

const MappedVector_psvbfx *frames_mvpsvb_as_storage(Frames_mvpsvbfx *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsbvfx *frames_mvpsbv_new(void);

/**
 * Frees the input instance.
 */
void frames_mvpsbv_free(Frames_mvpsbvfx *x);

/**
 * Serialize into json.
 */
void frames_mvpsbv_serialize(const Frames_mvpsbvfx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsbvfx *frames_mvpsbv_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void frames_mvpsbv_serialize_bin(const Frames_mvpsbvfx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsbvfx *frames_mvpsbv_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsbvfx *frames_mvpsbv_init(uintptr_t num_qubits);

void frames_mvpsbv_track_x(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_track_y(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_track_z(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_id(Frames_mvpsbvfx*, uintptr_t);

void frames_mvpsbv_x(Frames_mvpsbvfx*, uintptr_t);

void frames_mvpsbv_y(Frames_mvpsbvfx*, uintptr_t);

void frames_mvpsbv_z(Frames_mvpsbvfx*, uintptr_t);

void frames_mvpsbv_s(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sdg(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sz(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_szdg(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_hxy(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_h(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sy(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sydg(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sh(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_hs(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_shs(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sx(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_sxdg(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_hyz(Frames_mvpsbvfx *tracker, uintptr_t qubit);

void frames_mvpsbv_cz(Frames_mvpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsbv_cx(Frames_mvpsbvfx *tracker, uintptr_t control, uintptr_t target);

void frames_mvpsbv_cy(Frames_mvpsbvfx *tracker, uintptr_t control, uintptr_t target);

void frames_mvpsbv_swap(Frames_mvpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsbv_iswap(Frames_mvpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsbv_iswapdg(Frames_mvpsbvfx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void frames_mvpsbv_move_x_to_x(Frames_mvpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsbv_move_x_to_z(Frames_mvpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsbv_move_z_to_x(Frames_mvpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsbv_move_z_to_z(Frames_mvpsbvfx *tracker, uintptr_t source, uintptr_t destination);

void frames_mvpsbv_new_qubit(Frames_mvpsbvfx *tracker, uintptr_t qubit);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
PauliStack_bv *frames_mvpsbv_measure(Frames_mvpsbvfx *tracker,
                                     uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_psbv *frames_mvpsbv_stacked_transpose(Frames_mvpsbvfx *frames,
                                                     uintptr_t num_frames);

uintptr_t frames_mvpsbv_frames_num(Frames_mvpsbvfx *frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Frames_mvpsbvfx *frames_mvpsbv_new_unchecked(MappedVector_psbvfx *storage,
                                             uintptr_t num_frames);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
MappedVector_psbvfx *frames_mvpsbv_into_storage(Frames_mvpsbvfx *frames);

const MappedVector_psbvfx *frames_mvpsbv_as_storage(Frames_mvpsbvfx *frames);

void frames_hmpsvbfx_measure_and_store_hmfx(Frames_hmpsvbfx *frames,
                                            uintptr_t bit,
                                            Map_psvbfx *storage);

void frames_hmpsvbfx_measure_and_store_all_hmfx(Frames_hmpsvbfx *frames, Map_psvbfx *storage);

void frames_hmpsvbfx_measure_and_store_bv(Frames_hmpsvbfx *frames,
                                          uintptr_t bit,
                                          BufferedVector_psvb *storage);

void frames_hmpsvbfx_measure_and_store_all_bv(Frames_hmpsvbfx *frames,
                                              BufferedVector_psvb *storage);

void frames_hmpsvbfx_measure_and_store_mvfx(Frames_hmpsvbfx *frames,
                                            uintptr_t bit,
                                            MappedVector_psvbfx *storage);

void frames_hmpsvbfx_measure_and_store_all_mvfx(Frames_hmpsvbfx *frames,
                                                MappedVector_psvbfx *storage);

void frames_bvpsvb_measure_and_store_hmfx(Frames_bvpsvb *frames,
                                          uintptr_t bit,
                                          Map_psvbfx *storage);

void frames_bvpsvb_measure_and_store_all_hmfx(Frames_bvpsvb *frames, Map_psvbfx *storage);

void frames_bvpsvb_measure_and_store_bv(Frames_bvpsvb *frames,
                                        uintptr_t bit,
                                        BufferedVector_psvb *storage);

void frames_bvpsvb_measure_and_store_all_bv(Frames_bvpsvb *frames, BufferedVector_psvb *storage);

void frames_bvpsvb_measure_and_store_mvfx(Frames_bvpsvb *frames,
                                          uintptr_t bit,
                                          MappedVector_psvbfx *storage);

void frames_bvpsvb_measure_and_store_all_mvfx(Frames_bvpsvb *frames, MappedVector_psvbfx *storage);

void frames_mvpsvb_measure_and_store_hmfx(Frames_mvpsvbfx *frames,
                                          uintptr_t bit,
                                          Map_psvbfx *storage);

void frames_mvpsvb_measure_and_store_all_hmfx(Frames_mvpsvbfx *frames, Map_psvbfx *storage);

void frames_mvpsvb_measure_and_store_bv(Frames_mvpsvbfx *frames,
                                        uintptr_t bit,
                                        BufferedVector_psvb *storage);

void frames_mvpsvb_measure_and_store_all_bv(Frames_mvpsvbfx *frames, BufferedVector_psvb *storage);

void frames_mvpsvb_measure_and_store_mvfx(Frames_mvpsvbfx *frames,
                                          uintptr_t bit,
                                          MappedVector_psvbfx *storage);

void frames_mvpsvb_measure_and_store_all_mvfx(Frames_mvpsvbfx *frames,
                                              MappedVector_psvbfx *storage);

void frames_hmpsbvfx_measure_and_store_hmfx(Frames_hmpsbvfx *frames,
                                            uintptr_t bit,
                                            Map_psbvfx *storage);

void frames_hmpsbvfx_measure_and_store_all_hmfx(Frames_hmpsbvfx *frames, Map_psbvfx *storage);

void frames_hmpsbvfx_measure_and_store_bv(Frames_hmpsbvfx *frames,
                                          uintptr_t bit,
                                          BufferedVector_psbv *storage);

void frames_hmpsbvfx_measure_and_store_all_bv(Frames_hmpsbvfx *frames,
                                              BufferedVector_psbv *storage);

void frames_hmpsbvfx_measure_and_store_mvfx(Frames_hmpsbvfx *frames,
                                            uintptr_t bit,
                                            MappedVector_psbvfx *storage);

void frames_hmpsbvfx_measure_and_store_all_mvfx(Frames_hmpsbvfx *frames,
                                                MappedVector_psbvfx *storage);

void frames_bvpsbv_measure_and_store_hmfx(Frames_bvpsbv *frames,
                                          uintptr_t bit,
                                          Map_psbvfx *storage);

void frames_bvpsbv_measure_and_store_all_hmfx(Frames_bvpsbv *frames, Map_psbvfx *storage);

void frames_bvpsbv_measure_and_store_bv(Frames_bvpsbv *frames,
                                        uintptr_t bit,
                                        BufferedVector_psbv *storage);

void frames_bvpsbv_measure_and_store_all_bv(Frames_bvpsbv *frames, BufferedVector_psbv *storage);

void frames_bvpsbv_measure_and_store_mvfx(Frames_bvpsbv *frames,
                                          uintptr_t bit,
                                          MappedVector_psbvfx *storage);

void frames_bvpsbv_measure_and_store_all_mvfx(Frames_bvpsbv *frames, MappedVector_psbvfx *storage);

void frames_mvpsbv_measure_and_store_hmfx(Frames_mvpsbvfx *frames,
                                          uintptr_t bit,
                                          Map_psbvfx *storage);

void frames_mvpsbv_measure_and_store_all_hmfx(Frames_mvpsbvfx *frames, Map_psbvfx *storage);

void frames_mvpsbv_measure_and_store_bv(Frames_mvpsbvfx *frames,
                                        uintptr_t bit,
                                        BufferedVector_psbv *storage);

void frames_mvpsbv_measure_and_store_all_bv(Frames_mvpsbvfx *frames, BufferedVector_psbv *storage);

void frames_mvpsbv_measure_and_store_mvfx(Frames_mvpsbvfx *frames,
                                          uintptr_t bit,
                                          MappedVector_psbvfx *storage);

void frames_mvpsbv_measure_and_store_all_mvfx(Frames_mvpsbvfx *frames,
                                              MappedVector_psbvfx *storage);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_hmpefx *live_hmpefx_new(void);

/**
 * Frees the input instance.
 */
void live_hmpefx_free(Live_hmpefx *x);

/**
 * Serialize into json.
 */
void live_hmpefx_serialize(const Live_hmpefx *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_hmpefx *live_hmpefx_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void live_hmpefx_serialize_bin(const Live_hmpefx *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_hmpefx *live_hmpefx_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_hmpefx *live_hmpefx_init(uintptr_t num_qubits);

void live_hmpefx_track_x(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_track_y(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_track_z(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_id(Live_hmpefx*, uintptr_t);

void live_hmpefx_x(Live_hmpefx*, uintptr_t);

void live_hmpefx_y(Live_hmpefx*, uintptr_t);

void live_hmpefx_z(Live_hmpefx*, uintptr_t);

void live_hmpefx_s(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sdg(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sz(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_szdg(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_hxy(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_h(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sy(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sydg(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sh(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_hs(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_shs(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sx(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_sxdg(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_hyz(Live_hmpefx *tracker, uintptr_t qubit);

void live_hmpefx_cz(Live_hmpefx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_hmpefx_cx(Live_hmpefx *tracker, uintptr_t control, uintptr_t target);

void live_hmpefx_cy(Live_hmpefx *tracker, uintptr_t control, uintptr_t target);

void live_hmpefx_swap(Live_hmpefx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_hmpefx_iswap(Live_hmpefx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_hmpefx_iswapdg(Live_hmpefx *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_hmpefx_move_x_to_x(Live_hmpefx *tracker, uintptr_t source, uintptr_t destination);

void live_hmpefx_move_x_to_z(Live_hmpefx *tracker, uintptr_t source, uintptr_t destination);

void live_hmpefx_move_z_to_x(Live_hmpefx *tracker, uintptr_t source, uintptr_t destination);

void live_hmpefx_move_z_to_z(Live_hmpefx *tracker, uintptr_t source, uintptr_t destination);

void live_hmpefx_new_qubit(Live_hmpefx *tracker, uintptr_t qubit);

PauliEnum live_hmpefx_measure(Live_hmpefx *tracker, uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Map_pefx *live_hmpefx_into_storage(Live_hmpefx *frames);

const Map_pefx *live_hmpefx_as_storage(Live_hmpefx *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpe *live_bvpe_new(void);

/**
 * Frees the input instance.
 */
void live_bvpe_free(Live_bvpe *x);

/**
 * Serialize into json.
 */
void live_bvpe_serialize(const Live_bvpe *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpe *live_bvpe_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void live_bvpe_serialize_bin(const Live_bvpe *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpe *live_bvpe_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpe *live_bvpe_init(uintptr_t num_qubits);

void live_bvpe_track_x(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_track_y(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_track_z(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_id(Live_bvpe*, uintptr_t);

void live_bvpe_x(Live_bvpe*, uintptr_t);

void live_bvpe_y(Live_bvpe*, uintptr_t);

void live_bvpe_z(Live_bvpe*, uintptr_t);

void live_bvpe_s(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sdg(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sz(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_szdg(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_hxy(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_h(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sy(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sydg(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sh(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_hs(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_shs(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sx(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_sxdg(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_hyz(Live_bvpe *tracker, uintptr_t qubit);

void live_bvpe_cz(Live_bvpe *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpe_cx(Live_bvpe *tracker, uintptr_t control, uintptr_t target);

void live_bvpe_cy(Live_bvpe *tracker, uintptr_t control, uintptr_t target);

void live_bvpe_swap(Live_bvpe *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpe_iswap(Live_bvpe *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpe_iswapdg(Live_bvpe *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpe_move_x_to_x(Live_bvpe *tracker, uintptr_t source, uintptr_t destination);

void live_bvpe_move_x_to_z(Live_bvpe *tracker, uintptr_t source, uintptr_t destination);

void live_bvpe_move_z_to_x(Live_bvpe *tracker, uintptr_t source, uintptr_t destination);

void live_bvpe_move_z_to_z(Live_bvpe *tracker, uintptr_t source, uintptr_t destination);

void live_bvpe_new_qubit(Live_bvpe *tracker, uintptr_t qubit);

PauliEnum live_bvpe_measure(Live_bvpe *tracker, uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pe *live_bvpe_into_storage(Live_bvpe *frames);

const BufferedVector_pe *live_bvpe_as_storage(Live_bvpe *frames);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpt *live_bvpt_new(void);

/**
 * Frees the input instance.
 */
void live_bvpt_free(Live_bvpt *x);

/**
 * Serialize into json.
 */
void live_bvpt_serialize(const Live_bvpt *x, const char *file);

/**
 * Deserialize from json.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpt *live_bvpt_deserialize(const char *file);

/**
 * Serialize into binary code.
 */
void live_bvpt_serialize_bin(const Live_bvpt *x, const char *file);

/**
 * Deserialize from binary codeh.
 *
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpt *live_bvpt_deserialize_bin(const char *file);

/**
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
Live_bvpt *live_bvpt_init(uintptr_t num_qubits);

void live_bvpt_track_x(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_track_y(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_track_z(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_id(Live_bvpt*, uintptr_t);

void live_bvpt_x(Live_bvpt*, uintptr_t);

void live_bvpt_y(Live_bvpt*, uintptr_t);

void live_bvpt_z(Live_bvpt*, uintptr_t);

void live_bvpt_s(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sdg(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sz(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_szdg(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_hxy(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_h(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sy(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sydg(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sh(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_hs(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_shs(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sx(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_sxdg(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_hyz(Live_bvpt *tracker, uintptr_t qubit);

void live_bvpt_cz(Live_bvpt *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpt_cx(Live_bvpt *tracker, uintptr_t control, uintptr_t target);

void live_bvpt_cy(Live_bvpt *tracker, uintptr_t control, uintptr_t target);

void live_bvpt_swap(Live_bvpt *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpt_iswap(Live_bvpt *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpt_iswapdg(Live_bvpt *tracker, uintptr_t qubit_a, uintptr_t qubit_b);

void live_bvpt_move_x_to_x(Live_bvpt *tracker, uintptr_t source, uintptr_t destination);

void live_bvpt_move_x_to_z(Live_bvpt *tracker, uintptr_t source, uintptr_t destination);

void live_bvpt_move_z_to_x(Live_bvpt *tracker, uintptr_t source, uintptr_t destination);

void live_bvpt_move_z_to_z(Live_bvpt *tracker, uintptr_t source, uintptr_t destination);

void live_bvpt_new_qubit(Live_bvpt *tracker, uintptr_t qubit);

struct PauliTuple live_bvpt_measure(Live_bvpt *tracker, uintptr_t qubit);

/**
 * Frees the input instance.
 * The returned instance has to be freed manually with the according `*_free` function or indirecly with another function that consumes and frees it.
 */
BufferedVector_pt *live_bvpt_into_storage(Live_bvpt *frames);

const BufferedVector_pt *live_bvpt_as_storage(Live_bvpt *frames);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
