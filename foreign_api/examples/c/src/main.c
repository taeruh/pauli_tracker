#include <stdio.h>

#include "pauli_tracker.h"


int main(void) {
  Storage *storage = new_storage();
  put_some_stuff_into_storage(storage);
  RawStorage r_storage = raw_storage(storage);
  printf("qubit: %lu\n", *r_storage.inverse_position.ptr);
  RawPauliVec r_pauli_vec = raw_pauli_vec(r_storage.frames.ptr);
  printf("x correction %u\n", *r_pauli_vec.left.ptr);
  printf("z correction %u\n", *r_pauli_vec.right.ptr);
  free_storage(storage);
}
