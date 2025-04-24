#include <stdio.h>

extern "C" void cuda__add(int a, int b)
{
    printf("🔥 CUDA says: %d + %d = %d\n", a, b, a + b);
}
