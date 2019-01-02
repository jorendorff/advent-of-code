#include <iostream>
#include <inttypes.h>

using std::cout;
typedef uint64_t u64;

const u64 Width = 70;
const u64 Height = 800;
const u64 TargetX = 9;
const u64 TargetY = 751;
const u64 Depth = 11817;
const u64 Modulus = 20183;

u64 map[Height][Width];

u64 erosion(u64 gi) {
    return (gi + Depth) % Modulus;
}

int main() {
    map[0][0] = erosion(0);
    for (u64 x = 1; x <= Width; x++) {
        map[0][x] = erosion(x * 16807);
    }

    for (u64 y = 1; y <= Height; y++) {
        map[y][0] = erosion(y * 48271);
        for (u64 x = 1; x <= Width; x++) {
            if (y == TargetY && x == TargetX) {
                map[y][x] = erosion(0);
            } else {
                map[y][x] = erosion(map[y][x-1] * map[y-1][x]);
            }
        }
    }

    for (u64 y = 0; y < Height; y++) {
        for (u64 x = 0; x < Width; x++) {
            u64 risk = map[y][x] % 3;
            cout << ".=|"[risk];
        }
        cout << '\n';
    }

    return 0;
}
