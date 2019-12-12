"""
It is a six-digit number. The value is within the range given in your puzzle
input. Two adjacent digits are the same (like 22 in 122345). Going from left to
right, the digits never decrease; they only ever increase or stay the same
(like 111123 or 135679). Other than the range rule, the following are true:

111111 meets these criteria (double 11, never decreases). 223450 does not meet
these criteria (decreasing pair of digits 50). 123789 does not meet these
criteria (no double). How many different passwords within the range given in
your puzzle input meet these criteria?

Your puzzle input is 284639-748759.
"""
from collections import defaultdict


def two_same(in_list) -> bool:
    for i, ii in zip(in_list, in_list[1:]):
        if i == ii:
            return True
    return False


def digits_never_decrease(in_list) -> bool:
    min_value = "0"
    for i in in_list:
        if i < min_value:
            return False
        min_value = i
    return True


def check_number(num):
    return two_same(str(num)) and digits_never_decrease(str(num))


assert check_number(111111)
assert not check_number(223450)
assert not check_number(123789)

print(sum((check_number(i) for i in range(284639, 748759))))

"""
An Elf just remembered one more important detail: the two adjacent matching
digits are not part of a larger group of matching digits.
"""


def only_two_same(in_list) -> bool:
    cntr = defaultdict(lambda: 1)
    for i, ii in zip(in_list, in_list[1:]):
        if i == ii:
            cntr[i] += 1
    if 2 in cntr.values():
        return True
    return False


def check_number_2(num):
    return only_two_same(str(num)) and digits_never_decrease(str(num))


assert check_number_2(112233)
assert not check_number_2(123444)
assert check_number_2(111122)

print(sum((check_number_2(i) for i in range(284639, 748759))))
