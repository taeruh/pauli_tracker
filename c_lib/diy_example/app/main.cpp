#include "minimal_lib.h"
#include <stdio.h>

int main(void) {
  Live *live = create(2);
  track_x(live, 1);
  printf("qubit 0: %d\nqubit 1: %d\n", get(live, 0), get(live, 1));
  drop(live);
}
