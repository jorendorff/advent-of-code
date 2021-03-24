What would it take to write tools for intcode?

A proper disassembler might start disassembling at 0 and only
disassemble reached instructions, instead of unsoundly guessing.

Unpredictable control flow is a big problem here.

The problem is that indirect jumps, and particularly the "return"
instruction (`jump-if-true 1, rel[0]`)

There is a nice, standard iterate-to-a-fixed-point pattern that can be
used here. What is the machine state? It has to include some info about
the relative base, and some abstraction of what data is on stack at the
relative base (including the return pointer in some cases).

Some amount of self-modifying code has to be supported, for indirect
reads and even jumps. I don't know how to do that soundly.

Writes would have to be very carefully monitored to detect unsupported
patterns of self-modifying code.
