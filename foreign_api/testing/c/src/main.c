#include <stdio.h>

#include "pauli_tracker.h"

// void bar(void);

int main(void) {
  // bar();
  Storage *storage = new_storage();
  test(storage);
  RawStorage r_storage = raw_storage(storage);
  printf("%lu\n", *r_storage.inverse_position.ptr);
  RawPauliVec r_pauli_vec = raw_pauli_vec(r_storage.frames.ptr);
  printf("%u\n", *r_pauli_vec.left.ptr);
  printf("%u\n", *r_pauli_vec.right.ptr);
  free_storage(storage);
}

// void bar(void){}
