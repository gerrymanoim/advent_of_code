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
"""

from pathlib import Path
from typing import Callable


def run_program(input_ixs: list) -> list:
    cur = 0
    while True:
        # print(f"top of loop: {input_ixs}")
        ix = input_ixs[cur]
        if ix == 99:
            return input_ixs
        elif ix == 1:
            cur = operate(cur, input_ixs, lambda x, y: x + y)
        elif ix == 2:
            cur = operate(cur, input_ixs, lambda x, y: x * y)
        else:
            raise ValueError(f"Unknown ix. cur:{cur}, ixs:{input_ixs}")
    pass


def operate(cur: int, ixs: list, func: Callable) -> int:
    in_1 = ixs[ixs[cur + 1]]
    in_2 = ixs[ixs[cur + 2]]
    ixs[ixs[cur + 3]] = func(in_1, in_2)
    return cur + 4


assert run_program([1, 0, 0, 0, 99]) == [2, 0, 0, 0, 99]
assert run_program([2, 3, 0, 3, 99]) == [2, 3, 0, 6, 99]
assert run_program([2, 4, 4, 5, 99, 0]) == [2, 4, 4, 5, 99, 9801]
assert run_program([1, 1, 1, 4, 99, 5, 6, 0, 99]) == [
    30,
    1,
    1,
    4,
    2,
    5,
    6,
    0,
    99,
]

with Path("input2.txt") as f:
    instructions = f.read_text().split(",")
    instructions = [int(ix) for ix in instructions]
    """
    To do this, before running the program, replace position 1 with the value
    12 and replace position 2 with the value 2.
    """
    instructions[1] = 12
    instructions[2] = 2

print(run_program(instructions)[0])


with Path("input2.txt") as f:
    instructions = f.read_text().split(",")
    instructions = [int(ix) for ix in instructions]

for noun in range(0, 100):
    for verb in range(0, 100):
        memory = instructions.copy()
        memory[1] = noun
        memory[2] = verb
        out = run_program(memory)
        if out[0] == 19690720:
            print(100 * noun + verb)
            break
