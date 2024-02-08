#include <filesystem>
#include <stdio.h>

#include "pauli_tracker.h"

namespace fs = std::filesystem;

int main(void) {
  Map_psvbfx *storage = map_psvbfx_new();
  Frames_hmpsvbfx *frames = frames_hmpsvbfx_init(3);
  Live_hmpefx *live = live_hmpefx_init(8);
  Live_bvpt *tuple_live = live_bvpt_init(4);

  frames_hmpsvbfx_track_x(frames, 0);
  live_hmpefx_track_x(live, 0);
  live_bvpt_track_x(tuple_live, 0);

  frames_hmpsvbfx_cx(frames, 0, 1);
  live_hmpefx_cx(live, 0, 1);
  live_bvpt_cx(tuple_live, 0, 1);

  frames_hmpsvbfx_measure_and_store_hmfx(frames, 1, storage);
  printf("%d\n", *map_pefx_get(live_hmpefx_as_storage(live), 1));
  const PauliTuple *tuple_result =
      buffered_vector_pt_get(live_bvpt_as_storage(tuple_live), 1);
  printf("(%d, %d)\n", tuple_result->_0, tuple_result->_1);

  frames_hmpsvbfx_track_y(frames, 2);
  live_hmpefx_track_y(live, 2);
  live_bvpt_track_y(tuple_live, 2);

  frames_hmpsvbfx_measure_and_store_all_hmfx(frames, storage);

  size_t num_frames = frames_hmpsvbfx_frames_num(frames);

  fs::create_directories("output");
  map_psvbfx_serialize(storage, "output/frames.json");
  live_hmpefx_serialize(live, "output/live.json");
  live_bvpt_serialize(tuple_live, "output/tuple_live.json");

  // below we will transpose it which requires that all stacks have the same
  // length (required by *_new_unchecked)
  PauliStack_vb *stack = map_psvbfx_get_mut(storage, 1);
  vec_b_resize(pauli_stack_vb_x(stack), num_frames, false);
  vec_b_resize(pauli_stack_vb_z(stack), num_frames, false);

  size_t num_bits = map_psvbfx_len(storage);

  // frees storage, but we'll need to free frames_rebuilt
  Frames_hmpsvbfx *frames_rebuilt =
      frames_hmpsvbfx_new_unchecked(storage, num_frames);
  // frees frames_rebuilt, but we'll need to free transposed
  BufferedVector_psvb *transposed =
      frames_hmpsvbfx_stacked_transpose(frames_rebuilt, num_bits);
  buffered_vector_psvb_serialize(transposed, "output/frames_transposed.json");

  buffered_vector_psvb_free(transposed);
  frames_hmpsvbfx_free(frames);
  live_bvpt_free(tuple_live);
  live_hmpefx_free(live);

  Vec_b *v = vec_b_new();
  printf("%d\n", vec_b_is_empty(v));
  vec_b_free(v);

  return 0;
}
