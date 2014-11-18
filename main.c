#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
        int32_t x;
        int32_t y;
} vec2_t;

int32_t dubble(int32_t);
int32_t mathy(vec2_t);
int32_t refy(vec2_t*);
vec2_t clone_vec(vec2_t*);
vec2_t* new_vec(int32_t, int32_t);

int32_t get_x(vec2_t vec)
{
        return vec.x;
}

int32_t get_ry(vec2_t* vec)
{
        return vec->y;
}

void set_x(vec2_t* vec, int32_t x)
{
        vec->x = x;
}

int main()
{
        vec2_t vec = {
                .x = 2,
                .y = 3
        };
        int32_t value = 9;
        printf("9 dubble is %d\n", dubble(value));
        printf("mathy is %d\n", mathy(vec));
        refy(&vec);
        printf("refy did %d\n", mathy(vec));
        vec2_t cloned = clone_vec(&vec);
        printf("cloned: %d, %d\n", cloned.x, cloned.y);
        vec2_t *vec_p = new_vec(5, 7);
        printf("new: %d, %d\n", vec_p->x, vec_p->y);
        free(vec_p);
}
