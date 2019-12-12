"""
Your existing Intcode computer is missing one key feature: it needs support for
parameters in relative mode.

Parameters in mode 2, relative mode, behave very similarly to parameters in
position mode: the parameter is interpreted as a position. Like position mode,
parameters in relative mode can be read from or written to.

The important difference is that relative mode parameters don't count from
address 0. Instead, they count from a value called the relative base. The
relative base starts at 0.

The address a relative mode parameter refers to is itself plus the current
relative base. When the relative base is 0, relative mode parameters and
position mode parameters with the same value refer to the same address.

For example, given a relative base of 50, a relative mode parameter of -7
refers to memory address 50 + -7 = 43.

The relative base is modified with the relative base offset instruction:

Opcode 9 adjusts the relative base by the value of its only parameter. The
relative base increases (or decreases, if the value is negative) by the value
of the parameter. For example, if the relative base is 2000, then after the
instruction 109,19, the relative base would be 2019. If the next instruction
were 204,-34, then the value at address 1985 would be output.

Your Intcode computer will also need a few other capabilities:

The computer's available memory should be much larger than the initial program.
Memory beyond the initial program starts with the value 0 and can be read or
written like any other memory. (It is invalid to try to access memory at a
negative address, though.) The computer should have support for large numbers.
Some instructions near the beginning of the BOOST program will verify this
capability.
"""

from pathlib import Path
from intcode import Computer

in_1 = [
    str(x)
    for x in [
        109,
        1,
        204,
        -1,
        1001,
        100,
        1,
        100,
        1008,
        100,
        16,
        101,
        1006,
        101,
        0,
        99,
    ]
]
in_2 = [str(x) for x in [1102, 34915192, 34915192, 7, 4, 7, 99, 0]]
in_3 = [str(x) for x in [104, 1125899906842624, 99]]

assert Computer(in_1).run_until_halt() == [
    109,
    1,
    204,
    -1,
    1001,
    100,
    1,
    100,
    1008,
    100,
    16,
    101,
    1006,
    101,
    0,
    99,
]
assert len(str(Computer(in_2).run_until_halt()[0])) == 16
assert Computer(in_3).run_until_halt() == [1125899906842624]

in_txt = Path("input9.txt").read_text().rstrip().split(",")
print(Computer(in_txt, [1]).run_until_halt())
print(Computer(in_txt, [2]).run_until_halt())
