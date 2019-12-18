"""
An Intcode program, the Aft Scaffolding Control and Information Interface
(ASCII, your puzzle input), provides access to the cameras and the vacuum
robot. Currently, because the vacuum robot is asleep, you can only access the
cameras.

Running the ASCII program on your Intcode computer will provide the current
view of the scaffolds. This is output, purely coincidentally, as ASCII code: 35
means #, 46 means ., 10 starts a new line of output below the current one, and
so on. (Within a line, characters are drawn left-to-right.)

In the camera output, # represents a scaffold and . represents open space. The
vacuum robot is visible as ^, v, <, or > depending on whether it is facing up,
down, left, or right respectively. When drawn like this, the vacuum robot is
always on a scaffold; if the vacuum robot ever walks off of a scaffold and
begins tumbling through space uncontrollably, it will instead be visible as X.

The first step is to calibrate the cameras by getting the alignment parameters
of some well-defined points. Locate all scaffold intersections; for each, its
alignment parameter is the distance between its left edge and the left edge of
the view multiplied by the distance between its top edge and the top edge of
the view. Here, the intersections from the above image are marked O:

To calibrate the cameras, you need the sum of the alignment parameters. In the
above example, this is 76.

Run your ASCII program. What is the sum of the alignment parameters for the
scaffold intersections?


"""

from pathlib import Path
from intcode import Computer, read_intcode_program
from itertools import count

from typing import List

program = read_intcode_program(Path("input17.txt"))
aft_control = Computer(program)
screen = aft_control.run_until_halt()
char_screen = [chr(i) for i in screen]


def find_line_length(l: List[int]) -> int:
    for i, elem in enumerate(l):
        if elem == 10:
            return i


elements_per_line = find_line_length(screen) + 1


def find_alignment_points(screen, n):
    for i, _ in filter(lambda x: x[1] == 35, zip(count(n), screen[n:])):
        if (
            screen[i]
            == screen[i - 1]
            == screen[i + 1]
            == screen[i - n]
            == screen[i + n]
        ):
            yield (i % n) * (i // n)


print(sum(find_alignment_points(screen, elements_per_line)))
