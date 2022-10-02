#include <stdio.h>
#include <stdint.h>

int32_t multiply(int32_t a, int32_t b) {
    printf("[C] Hello from C!\n");
    printf("[C] Input a is: %i \n", a);
    printf("[C] Input b is: %i \n", b);
    printf("[C] Multiplying and returning result to Rust..\n");

    return a * b;
}
