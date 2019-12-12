"""
An Intcode program is a list of integers separated by commas (like 1,0,0,3,99).
To run one, start by looking at the first integer (called position 0). Here,
you will find an opcode - either 1, 2, or 99.

- 99 means that the program is finished and should immediately halt.
- Opcode 1 adds together numbers read from two positions and stores the result
  in a third position. The three integers immediately after the opcode tell you
  these three positions - the first two indicate the positions from which you
  should read the input values, and the third indicates the position at which
  the output should be stored.
- Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
  instead of adding them. Again, the three integers after the opcode indicate
  where the inputs and outputs are, not their values.
- Opcode 3 takes a single integer as input and saves it to the position given
  by its only parameter. For example, the instruction 3,50 would take an input
  value and store it at address 50.
- Opcode 4 outputs the value of its only parameter. For example, the
  instruction 4,50 would output the value at address 50.

Right now, your ship computer already understands parameter mode 0, position
mode, which causes the parameter to be interpreted as a position - if the
parameter is 50, its value is the value stored at address 50 in memory. Until
now, all parameters have been in position mode.

Now, your ship computer will also need to handle parameters in mode 1,
immediate mode. In immediate mode, a parameter is interpreted as a value - if
the parameter is 50, its value is simply 50.

Parameter modes are stored in the same value as the instruction's opcode. The
opcode is a two-digit number based only on the ones and tens digit of the
value, that is, the opcode is the rightmost two digits of the first value in an
instruction. Parameter modes are single digits, one per parameter, read
right-to-left from the opcode: the first parameter's mode is in the hundreds
digit, the second parameter's mode is in the thousands digit, the third
parameter's mode is in the ten-thousands digit, and so on. Any missing modes
are 0.


"""

from pathlib import Path
from typing import Callable
from collections import namedtuple


def f01(ixs, in_1, in_2, out_loc):
    ixs[out_loc] = str(in_1 + in_2)


def f02(ixs, in_1, in_2, out_loc):
    ixs[out_loc] = str(in_1 * in_2)


def f03(ixs, out_loc):
    ixs[out_loc] = input("Enter a number: ")


def f04(ixs, out):
    print(f">>>>> OUTPUT <<<<<< {out}")


def f05(ixs, in_1, in_2):
    if in_1 != 0:
        return in_2


def f06(ixs, in_1, in_2):
    if in_1 == 0:
        return in_2


def f07(ixs, in_1, in_2, out_loc):
    ixs[out_loc] = "1" if in_1 < in_2 else "0"


def f08(ixs, in_1, in_2, out_loc):
    ixs[out_loc] = "1" if in_1 == in_2 else "0"


Opt = namedtuple("Optcode", ["n_args", "func", "output"])

code_to_opt = {
    "99": "return",
    "01": Opt(2, f01, True),
    "02": Opt(2, f02, True),
    "03": Opt(0, f03, True),
    "04": Opt(1, f04, False),
    "05": Opt(2, f05, False),
    "06": Opt(2, f06, False),
    "07": Opt(2, f07, True),
    "08": Opt(2, f08, True),
}


def run_instruction(pntr: int, ixs: list) -> int:
    ix = ixs[pntr]
    if ix == "99":
        return -1
    opt_str, modes = ix[-2:].zfill(2), ix[:-2].zfill(3)[::-1]
    print(f"opt_str {opt_str}, modes {modes}")
    opt = code_to_opt[opt_str]

    buffer = []
    for i in range(opt.n_args):
        mode = modes[i]
        pos = int(ixs[pntr + i + 1])
        if mode == "1":
            buffer.append(pos)
        else:
            buffer.append(int(ixs[pos]))
    if opt.output:
        buffer.append(int(ixs[pntr + opt.n_args + 1]))

    print(f"buffer {buffer}")
    maybe_pntr = opt.func(ixs, *buffer)
    print(f"Maybe pnt {maybe_pntr}")
    if maybe_pntr:
        return maybe_pntr
    else:
        is_output = 1 if opt.output else 0
        return pntr + opt.n_args + is_output + 1


def run_program(input_ixs: list) -> list:
    pntr = 0
    while True:
        print(f"pntr: {pntr}")
        print(f"top of loop: {input_ixs}")
        pntr = run_instruction(pntr, input_ixs)
        if pntr == -1:
            return input_ixs


assert run_program(["1002", "4", "3", "4", "33"]) == [
    "1002",
    "4",
    "3",
    "4",
    "99",
]

with Path("input5.txt") as f:
    instructions = f.read_text().split(",")
    # instructions = [int(ix) for ix in instructions]

run_program(instructions)
