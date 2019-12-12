"""
The wires twist and turn, but the two wires occasionally cross paths. To fix
the circuit, you need to find the intersection point closest to the central
port. Because the wires are on a grid, use the Manhattan distance for this
measurement. While the wires do technically cross right at the central port
where they both start, this point does not count, nor does a wire count as
crossing with itself.
"""
from pathlib import Path
from collections import namedtuple

Point = namedtuple("Point", ["x", "y"])
Line = namedtuple("Line", ["start", "end"])
mover = {
    "U": lambda pnt, steps: Point(pnt.x, pnt.y + steps),
    "D": lambda pnt, steps: Point(pnt.x, pnt.y - steps),
    "L": lambda pnt, steps: Point(pnt.x - steps, pnt.y),
    "R": lambda pnt, steps: Point(pnt.x + steps, pnt.y),
}


def ixs_to_points(ixs: list) -> list:
    out = [Point(0, 0)]
    for ix in ixs:
        direction, steps = ix[0], int(ix[1:])
        current_loc = out[-1]
        out.append(mover[direction](current_loc, steps))
    return out


def points_to_lines(points: list) -> list:
    return [Line(start, end) for start, end in zip(points, points[1:])]


def ixs_to_lines(ixs: list) -> list:
    points = ixs_to_points(ixs)
    return points_to_lines(points)


def find_closest_intersection(ixs_1: list, ixs_2: list) -> int:
    lines_1 = ixs_to_lines(ixs_1)
    lines_2 = ixs_to_lines(ixs_2)
    min_dist = None
    for l1 in lines_1:
        for l2 in lines_2:
            if check_if_intersecting(l1, l2):
                dist = get_intersect_distance(l1, l2)
                print(f"new dist {dist}, min_dist: {min_dist}")
                min_dist = dist if not min_dist else min(min_dist, dist)
    return min_dist


def check_if_intersecting(l1: Line, l2: Line) -> bool:
    # print(f"l1: {l1}, l2: {l2}")
    if l1.start.y == l1.end.y:
        if is_between(l1.start.x, l1.end.x, l2.start.x):
            if l1.start.y < l2.start.y and l1.start.y > l2.end.y:
                return True
            elif l1.start.y > l2.start.y and l1.start.y < l2.end.y:
                return True
            else:
                return False
        else:
            return False
    elif l2.start.y == l2.end.y:
        return check_if_intersecting(l2, l1)
    else:
        # flip orientation
        return check_if_intersecting(rotate_line(l1), rotate_line(l2))


def is_between(p1, p2, p3):
    left = min(p1, p2)
    right = max(p1, p2)
    return left <= p3 <= right


def rotate_line(line: Line) -> Line:
    return Line(
        start=Point(line.start.y, line.start.x),
        end=Point(line.end.y, line.end.x),
    )


def get_intersection_point(l1: Line, l2: Line) -> Point:
    if l1.start.y == l1.end.y:
        return Point(x=l2.start.x, y=l1.start.y)
    else:
        return get_intersection_point(l2, l1)


def get_intersect_distance(l1: Line, l2: Line) -> int:
    ix_point = get_intersection_point(l1, l2)
    return abs(ix_point.x) + abs(ix_point.y)


assert (
    find_closest_intersection(
        ["R8", "U5", "L5", "D3"], ["U7", "R6", "D4", "L4"]
    )
    == 6
)

assert (
    find_closest_intersection(
        ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
        ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    )
    == 159
)

assert (
    find_closest_intersection(
        [
            "R98",
            "U47",
            "R26",
            "D63",
            "R33",
            "U87",
            "L62",
            "D20",
            "R33",
            "U53",
            "R51",
        ],
        ["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"],
    )
    == 135
)

with Path("input3.txt").open() as f:
    ix_1 = f.readline().split(",")
    ix_2 = f.readline().split(",")
    print(find_closest_intersection(ix_1, ix_2))


"""
It turns out that this circuit is very timing-sensitive; you actually need to
minimize the signal delay.

To do this, calculate the number of steps each wire takes to reach each
intersection; choose the intersection where the sum of both wires' steps is
lowest. If a wire visits a position on the grid multiple times, use the steps
value from the first time it visits that position when calculating the total
value of a specific intersection.

The number of steps a wire takes is the total number of grid squares the wire
has entered to get to that location, including the intersection being
considered.
"""


def find_min_steps(ixs_1: list, ixs_2: list) -> int:
    lines_1 = ixs_to_lines(ixs_1)
    steps_1 = [int(ix[1:]) for ix in ixs_1]
    lines_2 = ixs_to_lines(ixs_2)
    steps_2 = [int(ix[1:]) for ix in ixs_2]
    min_steps = 0
    for l1_idx, l1 in enumerate(lines_1):
        for l2_idx, l2 in enumerate(lines_2):
            if check_if_intersecting(l1, l2):
                ix_point = get_intersection_point(l1, l2)
                dist_1 = distance_between_points(l1.start, ix_point)
                dist_2 = distance_between_points(l2.start, ix_point)
                steps = (
                    sum(steps_1[:l1_idx])
                    + sum(steps_2[:l2_idx])
                    + dist_1
                    + dist_2
                )
                print(f"new steps {steps}, min_steps: {min_steps}")
                min_steps = steps if not min_steps else min(min_steps, steps)
    return min_steps


def distance_between_points(source: Point, target: Point) -> int:
    # we know that target and source have a x or y in common
    if source.x == target.x:
        return abs(source.y - target.y)
    else:
        return abs(source.x - target.x)


assert find_min_steps(["R8", "U5", "L5", "D3"], ["U7", "R6", "D4", "L4"]) == 30


with Path("input3.txt").open() as f:
    ix_1 = f.readline().split(",")
    ix_2 = f.readline().split(",")
    print(find_min_steps(ix_1, ix_2))
