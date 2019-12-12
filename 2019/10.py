"""
The map indicates whether each position is empty (.) or contains an asteroid
(#). The asteroids are much smaller than they appear on the map, and every
asteroid is exactly in the center of its marked position. The asteroids can be
described with X,Y coordinates where X is the distance from the left edge and Y
is the distance from the top edge (so the top-left corner is 0,0 and the
position immediately to its right is 1,0).

Your job is to figure out which asteroid would be the best place to build a new
monitoring station. A monitoring station can detect any asteroid to which it
has direct line of sight - that is, there cannot be another asteroid exactly
between them. This line of sight can be at any angle, not just lines aligned to
the grid or diagonally. The best location is the asteroid that can detect the
largest number of other asteroids.
"""

from pathlib import Path
from itertools import groupby
from cmath import pi
from math import atan2
from typing import List, Tuple
from collections import deque


class Point:
    x_offset: int = 0
    y_offset: int = 0

    def __init__(self, x: int, y: int):
        self._x = x
        self._y = y

    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"{self.xy} =({self._x}, {self._y})="

    @property
    def x(self):
        return self._x - self.x_offset

    @property
    def y(self):
        return self._y - self.y_offset

    @property
    def xy(self):
        return (self.x, self.y)

    @property
    def slope(self):
        return None if self.x == 0 else self.y / self.x

    @property
    def dist(self):
        return (self.x ** 2 + self.y ** 2) ** 0.5

    @property
    def is_origin(self):
        return self.x == 0 and self.y == 0

    @property
    def theta(self):
        return atan2(self.y, self.x)

    @property
    def rotated_theta(self):
        """
        Rotate the point 90 degrees counter clockwise and get theta
        """
        return atan2(self.x, -self.y)


def parse_input(in_str: str) -> List[Point]:
    out = []
    for y, line in enumerate(in_str.splitlines()):
        for x, loc in enumerate(line):
            if loc == "#":
                out.append(Point(x, -y))
    return out


def count_detections(origin: Point, points: List[Point]) -> int:
    # change the cordinate system to be relative to origin point
    Point.x_offset, Point.y_offset = origin.xy

    # Don't care which is closest, just the unique count
    detections = len(
        set([point.theta for point in points if not point.is_origin])
    )
    Point.x_offset, Point.y_offset = (0, 0)
    return detections


def find_max_visible(points: List[Point]) -> Tuple[int, Tuple[int, int]]:
    found_max = (0, None)
    for origin in points:
        detections = count_detections(origin, points)
        if detections > found_max[0]:
            found_max = (detections, origin.xy)

    return found_max


in_1 = parse_input(Path("test10_1.txt").read_text().rstrip())
assert find_max_visible(in_1) == (8, (3, -4))

in_2 = """......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
""".rstrip()
assert find_max_visible(parse_input(in_2)) == (33, (5, -8))

asteroids = parse_input(Path("input10.txt").read_text().rstrip())
part_1 = find_max_visible(asteroids)
print(part_1[0])

base = Point(*part_1[1])


def order_of_vaprization(
    base_location: Point, points: List[Point], up_to_n: int
) -> List[Point]:
    Point.x_offset, Point.y_offset = base_location.xy
    # we rotate to make the theta sort work correctly
    others = [point for point in points if not point.is_origin]
    others.sort(key=lambda x: (-x.rotated_theta, x.dist))

    groups = deque()
    unique_keys = deque()
    for k, g in groupby(others, key=lambda x: -1 * x.rotated_theta):
        groups.append(deque(g))
        unique_keys.append(k)
    out = []
    first_pass = True
    for _ in range(up_to_n):
        key = unique_keys.popleft()
        group = groups.popleft()
        if first_pass:
            if key > pi:
                unique_keys.append(key)
                groups.append(group)
                continue
            else:
                first_pass = False
        out.append(group.popleft())
        if len(group) > 0:
            unique_keys.append(key)
            groups.append(group)

    Point.x_offset, Point.y_offset = (0, 0)
    return out


vaporized = order_of_vaprization(base, asteroids, 200)
two_hundred = vaporized[199]
print(two_hundred.x * 100 + two_hundred.y * -1)
