"""Search the puzzle input for strings.

The routine at 1234 decodes and prints a string. We attempt this for every
address in the puzzle input and print the results that seem likely to be
strings.

This finds some spurious candidates nested within real strings.
"""

import sys
from ad2019.intcode.interpreter import load

program = load("ad2019/dec25/puzzle-input.txt")

# fn for_each(
#     mut a1: &Array<Int>,
#     f: fn (Int, Int, Int),
# ) {
#     let n = a1.length;
#     for i in 0..n {
#         call f(a1.data[i], i, n);
#     }
# }
#
# // at 1234
# fn out_phrase(a1: &Array<Int>) {
#     call for_each(a1, out_sum);
# }
#
# // at 1256
# fn out_sum(a1: Int, a2: Int, a3: Int) {
#     r1 = a1 + a2 + a3;
#     print!("{}", unsafe { r1 as char });
# }


def decode_encrypted_str(program, addr):
    n = program[addr]
    if 1 <= n <= 1000 and addr + 1 + n <= len(program):
        s = ""
        for i in range(n):
            code = program[addr + 1 + i] + i + n
            if not (9 <= code <= 126) or code in (12, 13, 19):
                break
            s += chr(code)
        else:
            if '\\' in repr(s):
                s = repr(s)
            print(f"{addr}..{addr + 1 + n}: {s}")


for addr in range(len(program)):
    decode_encrypted_str(program, addr)

