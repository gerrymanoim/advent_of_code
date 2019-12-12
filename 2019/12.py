"""
After a brief scan, you calculate the position of each moon (your puzzle
input). You just need to simulate their motion so you can avoid them.

Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional
velocity. The position of each moon is given in your scan; the x, y, and z
velocity of each moon starts at 0.

Simulate the motion of the moons in time steps. Within each time step, first
update the velocity of every moon by applying gravity. Then, once all moons'
velocities have been updated, update the position of every moon by applying
velocity. Time progresses by one step once all of the positions are updated.

To apply gravity, consider every pair of moons. On each axis (x, y, and z), the
velocity of each moon changes by exactly +1 or -1 to pull the moons together.
For example, if Ganymede has an x position of 3, and Callisto has a x position
of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and Callisto's x
velocity changes by -1 (because 3 < 5). However, if the positions on a given
axis are the same, the velocity on that axis does not change for that pair of
moons.

Once all gravity has been applied, apply velocity: simply add the velocity of
each moon to its own position. For example, if Europa has a position of x=1,
y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be x=-1,
y=2, z=6. This process does not modify the velocity of any moon.
"""
from pathlib import Path
from collections import namedtuple
from itertools import combinations


Position = namedtuple("Position", ["x", "y", "z"])
Velocity = namedtuple("Velocity", ["x", "y", "z"])


class Moon:
    def __init__(self, x: int, y: int, z: int):
        self.position = Position(x, y, z)
        self._velocity = Velocity(0, 0, 0)

    def __call__(self):
        # step
        pass

    def __xor__(self, other):
        self.velocity = other.position
        other.velocity = self.position

    @property
    def velocity(self):
        return self._velocity

    @velocity.setter
    def velocity(self, other_position):
        for dim in ("x", "y", "z"):
            self._velocity["dim"] = self.velocity["dim"] + self.position_delta(
                self.position["dim"], other_position["dim"]
            )
        # update our velocity in position comparison

    def position_delta(self, mine, other):
        if mine > other:
            return 1
        elif mine < other:
            return -1
        else:
            return 0
