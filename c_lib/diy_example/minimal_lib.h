#include <stdint.h>

typedef struct Live Live;

#ifdef __cplusplus
extern "C" {
#endif

Live *create(uintptr_t size);
void drop(Live *live);
void track_x(Live *live, uintptr_t x);
char get(const Live *live, uintptr_t x);

#ifdef __cplusplus
}
#endif
