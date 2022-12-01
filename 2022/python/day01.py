from pathlib import Path
import heapq

input_file = Path(__file__).parent.parent / "inputs" / "input01.txt"
calories = input_file.read_text().splitlines()

def part_1(v: list[str]) -> int:
    max_seen = 0
    current_sum = 0
    for s in v:
        if s == "":
            if current_sum > max_seen:
                max_seen = current_sum
            current_sum = 0
        else:
            current_sum += int(s)
    if current_sum > max_seen:
        return current_sum
    else:
        return max_seen


def part_2(v: list[str]) -> int:
    current_sum = 0
    sums = list()
    for s in v:
        if s == "":
            sums.append(current_sum)
            current_sum = 0
        else:
            current_sum += int(s)
    sums.append(current_sum)
    return sum(heapq.nlargest(3, sums))


print(part_1(calories))
print(part_2(calories))
