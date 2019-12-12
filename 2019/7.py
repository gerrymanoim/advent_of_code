"""
When a copy of the program starts running on an amplifier, it will first use an
input instruction to ask the amplifier for its current phase setting (an
integer from 0 to 4). Each phase setting is used exactly once, but the Elves
can't remember which amplifier needs which phase setting.

The program will then call another input instruction to get the amplifier's
input signal, compute the correct output signal, and supply it back to the
amplifier with an output instruction. (If the amplifier has not yet received an
input signal, it waits until one arrives.)

Your job is to find the largest output signal that can be sent to the thrusters
by trying every possible combination of phase settings on the amplifiers. Make
sure that memory is not shared or reused between copies of the program.
"""
from typing import List
from intcode import Computer
from itertools import permutations
from pathlib import Path


def get_amplifiers(ixs, n_amps: int = 5, debug_mode: bool = False) -> List[Computer]:
    return [Computer(ixs, None, debug_mode) for _ in range(5)]


def run_1(input_ixs):
    max_output = 0
    max_trial = None
    n_amps = 5
    for trial in permutations([0, 1, 2, 3, 4], 5):
        amps = get_amplifiers(input_ixs, n_amps)
        prev_output = 0
        for order, amp in enumerate(amps):
            amp << trial[order]
            amp << prev_output
            prev_output = amp()
        if prev_output > max_output:
            max_output = prev_output
            max_trial = trial
    return max_output, max_trial


assert run_1(
    [
        str(x)
        for x in [
            3,
            15,
            3,
            16,
            1002,
            16,
            10,
            16,
            1,
            16,
            15,
            15,
            4,
            15,
            99,
            0,
            0,
        ]
    ]
) == (43210, (4, 3, 2, 1, 0))

assert run_1(
    [
        str(x)
        for x in [
            3,
            23,
            3,
            24,
            1002,
            24,
            10,
            24,
            1002,
            23,
            -1,
            23,
            101,
            5,
            23,
            23,
            1,
            24,
            23,
            23,
            4,
            23,
            99,
            0,
            0,
        ]
    ]
) == (54321, (0, 1, 2, 3, 4))

assert run_1(
    [
        str(x)
        for x in [
            3,
            31,
            3,
            32,
            1002,
            32,
            10,
            32,
            1001,
            31,
            -2,
            31,
            1007,
            31,
            0,
            33,
            1002,
            33,
            7,
            33,
            1,
            33,
            31,
            31,
            1,
            32,
            31,
            31,
            4,
            31,
            99,
            0,
            0,
            0,
        ]
    ]
) == (65210, (1, 0, 4, 3, 2))

with Path("input7.txt") as f:
    instructions = f.read_text().split(",")


print(run_1(instructions))


def run_2(input_ixs, debug=False):
    max_output = 0
    max_trial = None
    n_amps = 5
    for trial in permutations([5, 6, 7, 8, 9], 5):
        amps = get_amplifiers(input_ixs, n_amps, debug_mode=debug)
        prev_output = 0
        for order, amp in enumerate(amps):
            amp << trial[order]
        while True:
            for amp in amps:
                if amp.is_halted:
                    continue
                amp << prev_output
                prev_output = amp()
            if all([amp.is_halted for amp in amps]):
                break
        if prev_output > max_output:
            max_output = prev_output
            max_trial = trial
    return max_output, max_trial


assert run_2([str(x) for x in [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]]) == (139629729, (9, 8, 7, 6, 5))

with Path("test7_2.txt") as f:
    instructions = f.read_text().rstrip().split(",")


assert run_2(instructions) == (18216, (9, 7, 8, 5, 6))


with Path("input7.txt") as f:
    instructions = f.read_text().rstrip().split(",")


print(run_2(instructions))