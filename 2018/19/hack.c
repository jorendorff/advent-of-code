#include <stdio.h>

int f(R0)
{
    int R1 = 0, R2 = 0, R4 = 0, R5 = 0;

    R2 = 940;
    if (R0 == 1) {
        R2 = 753580;
    }

    R0 = 0;

    for (R4 = 1; R4 <= R2; R4++) {
        for (R1 = 1; R1 <= R2; R1++) {
            if (R4 * R1 == R2)
                R0 += R4;
        }
    }

    return R0;
}

int main() {
    printf("%d\n", f(0));
    printf("%d\n", f(1));
    return 0;
}

