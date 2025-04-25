#include <stdio.h>

extern "C" void add(int a, int b)
{
    printf("🔥 CUDA says: %d + %d = %d\n", a, b, a + b);
}
