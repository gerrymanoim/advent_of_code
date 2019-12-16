"""
The remote control program executes the following steps in a loop forever:

- Accept a movement command via an input instruction.
- Send the movement command to the repair droid.
- Wait for the repair droid to finish the movement operation.
- Report on the status of the repair droid via an output instruction.

Only four movement commands are understood: north (1), south (2), west (3), and
east (4). Any other command is invalid. The movements differ in direction, but
not in distance: in a long enough east-west hallway, a series of commands like
4,4,4,4,3,3,3,3 would leave the repair droid back where it started.

The repair droid can reply with any of the following status codes:

- 0: The repair droid hit a wall. Its position has not changed.
- 1: The repair droid has moved one step in the requested direction.
- 2: The repair droid has moved one step in the requested direction; its new
  position is the location of the oxygen system.
"""
from pathlib import Path
from intcode import Computer, read_intcode_program, Point
from collections import namedtuple
from copy import deepcopy

from typing import Dict

DroidState = namedtuple("DroidState", ["loc", "direction", "n_steps"])


class Droid:

    direction_codes = {"north": "1", "south": "2", "west": "3", "east": "4"}
    transition_fn = {
        "north": lambda p: Point(p.x, p.y + 1),
        "south": lambda p: Point(p.x, p.y - 1),
        "west": lambda p: Point(p.x - 1, p.y),
        "east": lambda p: Point(p.x + 1, p.y),
    }
    number_to_tile = {0: "#", 1: " ", 2: "O"}

    def __init__(self, start_point: Point, start_state: Computer):
        self.visited_points = set([start_point])
        self.points_to_visit: Dict[Point, DroidState] = {}
        self.add_points_to_visit(start_point, 0)
        self.state = {start_point: start_state}
        self.oxygen = None
        self.furthest_traveled = 0

    def find_oxygen(self) -> int:
        while self.oxygen is None:
            self.explore()
        return self.oxygen

    def find_steps_to_fill(self) -> int:
        while len(self.points_to_visit) > 0:
            self.explore()
        return self.furthest_traveled

    def explore(self):
        for point in list(self.points_to_visit.keys()):
            state = self.points_to_visit.pop(point)
            computer = deepcopy(self.state[state.loc])
            computer << self.direction_codes[state.direction]
            reply = computer()
            self.visited_points.add(point)
            if reply == 0:
                # wall
                pass
            elif reply == 1:
                self.state[point] = computer
                self.add_points_to_visit(point, state.n_steps)
                self.update_furthest_traveled(state.n_steps)
            elif reply == 2:
                self.state[point] = computer
                self.oxygen = DroidState(point, None, state.n_steps)
            else:
                raise ValueError(f"Got a weird reply from the drone {reply}")

    def add_points_to_visit(self, point: Point, n_steps: int):
        for direction in self.direction_codes.keys():
            fn = self.transition_fn[direction]
            new_point = fn(point)
            if new_point not in self.visited_points:
                self.points_to_visit[new_point] = DroidState(
                    point, direction, n_steps + 1
                )

    def update_furthest_traveled(self, n_steps):
        if self.furthest_traveled < n_steps:
            self.furthest_traveled = n_steps


in_txt = read_intcode_program(Path("input15.txt"))
droid = Droid(Point(0, 0), Computer(in_txt))
oxygen = droid.find_oxygen()
print(f"Part 1: {oxygen.n_steps}")
oxygen_state = droid.state[oxygen.loc]
droid = Droid(oxygen.loc, oxygen_state)
steps = droid.find_steps_to_fill()
print(f"Part 2: {steps}")
