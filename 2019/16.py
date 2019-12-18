"""
As input, FFT takes a list of numbers. In the signal you received (your puzzle
input), each number is a single digit: data like 15243 represents the sequence
1, 5, 2, 4, 3.

FFT operates in repeated phases. In each phase, a new list is constructed with
the same length as the input list. This new list is also used as the input for
the next phase.

Each element in the new list is built by multiplying every value in the input
list by a value in a repeating pattern and then adding up the results. So, if
the input list were 9, 8, 7, 6, 5 and the pattern for a given element were 1,
2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 + 5*2 (with each input element
on the left and each value in the repeating pattern on the right of each
multiplication). Then, only the ones digit is kept: 38 becomes 8, -17 becomes
7, and so on.

While each element in the output array uses all of the same input array
elements, the actual repeating pattern to use depends on which output element
is being calculated. The base pattern is 0, 1, 0, -1. Then, repeat each value
in the pattern a number of times equal to the position in the output list being
considered. Repeat once for the first element, twice for the second element,
three times for the third element, and so on. So, if the third element of the
output list is being calculated, repeating the values would produce: 0, 0, 0,
1, 1, 1, 0, 0, 0, -1, -1, -1.

When applying the pattern, skip the very first value exactly once. (In other
words, offset the whole pattern left by one.) So, for the second element of the
output list, the actual pattern used would be: 0, 1, 1, 0, 0, -1, -1, 0, 0, 1,
1, 0, 0, -1, -1, ....

After using this process to calculate each element of the output list, the
phase is complete, and the output list of this phase is used as the new input
list for the next phase, if any.
"""
from itertools import cycle, repeat, chain
from pathlib import Path
from typing import List

BASE = [0, 1, 0, -1]


def str_to_digits(text: str) -> List[int]:
    return [int(char) for char in text]


def calculate_digit(numbers: List[int], position: int) -> int:
    it = cycle(chain.from_iterable((repeat(d, position) for d in BASE)))
    next(it)
    return abs(sum((x * y for x, y in zip(numbers, it)))) % 10


def run_phase(numbers: List[int]) -> List[int]:
    return [
        calculate_digit(numbers, position)
        for position in range(1, len(numbers) + 1)
    ]


def run_n_phases(numbers: List[int], n_times: int) -> List[int]:
    for _ in range(n_times):
        numbers = run_phase(numbers)
    return numbers


test_input = str_to_digits("12345678")

assert run_n_phases(test_input, 4) == str_to_digits("01029498")

in_list = str_to_digits(Path("input16.txt").read_text().rstrip())
part_1 = run_n_phases(in_list, 100)
print(part_1[:8])

"""
The real signal is your puzzle input repeated 10000 times. Treat this new
signal as a single input list. Patterns are still calculated as before, and 100
phases of FFT are still applied.

The first seven digits of your initial input signal also represent the message
offset. The message offset is the location of the eight-digit message in the
final output list. Specifically, the message offset indicates the number of
digits to skip before reading the eight-digit message. For example, if the
first seven digits of your initial input signal were 1234567, the eight-digit
message would be the eight digits after skipping 1,234,567 digits of the final
output list. Or, if the message offset were 7 and your final output list were
98765432109876543210, the eight-digit message would be 21098765. (Of course,
your real message offset will be a seven-digit number, not a one-digit number
like 7.)

Here is the eight-digit message in the final output list after 100 phases. The
message offset given in each input has been highlighted. (Note that the inputs
given below are repeated 10000 times to find the actual starting input lists.)

After repeating your input signal 10000 times and running 100 phases of FFT,
what is the eight-digit message embedded in the final output list?
"""

# message_offset = sum(in_list[:7])
# read_from = message_offset + 1
# read_to = read_from + 8

# part_2_input = in_list*10_000
# part_2 = run_n_phases(part_2_input, 100)
# print(part_2[read_from:read_to])
