#include <stdio.h>

#include <inttypes.h>

typedef uint64_t u64;

uint8_t seen[0x1000000];

int main() {
    u64 r1 = 0; // seti 0 7 1
    u64 prev = -1;
    do {
        u64 r4 = r1 | 0x10000; // bori 1 65536 4
        r1 = 3798839;  // seti 3798839 3 1
        while (1) {
            // bani 4 255 5
            r1 += r4 & 0xff; // addr 1 5 1
            r1 &= 0xffffff; // bani 1 16777215 1
            r1 *= 65899; // muli 1 65899 1
            r1 &= 0xffffff; // bani 1 16777215 1
            if (256 > r4) // gtir 256 4 5
                // addr 5 3 3
                // addi 3 1 3
                break; // seti 27 6 3

            // compute the minimum r5>=0 such that (r5 + 1) * 256 > r4.
            r4 >>= 8;
            // for (r5 = 0; ; r5++) { // seti 0 2 5
            //     r2 = r5 + 1; // addi 5 1 2
            //     r2 *= 256; // muli 2 256 2
            //     if (r2 > r4) // gtrr 2 4 2
            //         // addr 2 3 3
            //         // addi 3 1 3
            //         break; // seti 25 3 3
            //     // addi 5 1 5
            // } // seti 17 1 3
            // 
            // r4 = r5; // setr 5 6 4
        }
        printf("r1=%llx\n", r1);
        if (seen[r1])
            break;
        seen[r1] = 1;
    } while (1);

    return 0;
}

