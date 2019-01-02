#include <iostream>
#include <inttypes.h>

using std::cout;
typedef uint64_t u64;

const u64 Width = 30;
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
    u64 total = 0;
    map[0][0] = erosion(0);
    for (u64 x = 1; x <= TargetX; x++) {
        map[0][x] = erosion(x * 16807);
        total += map[0][x] % 3;
    }

    for (u64 y = 1; y <= TargetY; y++) {
        map[y][0] = erosion(y * 48271);
        total += map[y][0] % 3;
        for (u64 x = 1; x <= TargetX; x++) {
            if (y == TargetY && x == TargetX) {
                map[y][x] = erosion(0);
            } else {
                map[y][x] = erosion(map[y][x-1] * map[y-1][x]);
            }
            total += map[y][x] % 3;
        }
    }

    cout << total << '\n';
    return 0;
}
