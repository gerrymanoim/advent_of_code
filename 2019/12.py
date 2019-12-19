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
from functools import reduce
from itertools import combinations
import re
from string import ascii_uppercase
from typing import Iterator, List, Dict
from math import gcd


class Moon:
    dimensions: List[str] = ["x", "y", "z"]

    def __init__(self, x: int, y: int, z: int, name: str):
        self.position = dict(x=x, y=y, z=z)
        self.velocity = dict(x=0, y=0, z=0)
        self.name = name

    def __repr__(self):
        return str(self)

    def __str__(self):
        return f"pos=<{self.position}>, vel=<{self.velocity}>"

    def __xor__(self, other):
        for dim in ("x", "y", "z"):
            self.velocity[dim] += self.velocity_delta(
                self.position[dim], other.position[dim]
            )
            other.velocity[dim] += other.velocity_delta(
                other.position[dim], self.position[dim]
            )

    def move(self):
        for dim in self.dimensions:
            self.position[dim] += self.velocity[dim]

    def velocity_delta(self, mine, other):
        if mine > other:
            return -1
        elif mine < other:
            return 1
        else:
            return 0

    @property
    def kinetic_energy(self) -> int:
        return sum((abs(coord) for coord in self.velocity.values()))

    @property
    def potential_energy(self) -> int:
        return sum((abs(coord) for coord in self.position.values()))

    @property
    def total_energy(self) -> int:
        return self.kinetic_energy * self.potential_energy

    @property
    def info(self) -> Dict[str, int]:
        return {
            "name": self.name,
            "x": self.position["x"],
            "y": self.position["y"],
            "z": self.position["z"],
            "vx": self.velocity["x"],
            "vy": self.velocity["y"],
            "vz": self.velocity["z"],
            "ke": self.kinetic_energy,
            "pe": self.potential_energy,
            "te": self.total_energy,
        }


def extract_coordinates(txt: str) -> Iterator[int]:
    coordinates = re.findall(r"-?\d{1,2}", txt)
    return (int(coordinate) for coordinate in coordinates)


in_txt = Path("input12.txt").read_text().rstrip().splitlines()

moons = [
    Moon(*extract_coordinates(moon), name)
    for name, moon in zip(ascii_uppercase, in_txt)
]


def energy_after_n_steps(moons: List[Moon], steps: int = 1000) -> int:
    for _ in range(steps):
        for left, right in combinations(moons, 2):
            left ^ right
        _ = [moon.move() for moon in moons]

    return sum((moon.total_energy for moon in moons))


print(energy_after_n_steps(moons))

"""
All this drifting around in space makes you wonder about the nature of the
universe. Does history really repeat itself? You're curious whether the moons
will ever return to a previous state.

Determine the number of steps that must occur before all of the moons'
positions and velocities exactly match a previous point in time.

NOTEs (for self):
- sum of all velocities in a direction == 0
- sum of all positions in a dimension == some constant

- each individual velocity is on some cycle
- So all are on some LCM cycle
- So positions are on some cycle.
"""

moons = [
    Moon(*extract_coordinates(moon), name)
    for name, moon in zip(ascii_uppercase, in_txt)
]


def lcm(x: int, y: int) -> int:
    return x * y // gcd(x, y)


def search_space(moons: List[Moon]) -> Dict[str, int]:
    states = {dim: set() for dim in Moon.dimensions}
    steps = 0
    out = {}
    while len(out.values()) != 3:
        state = {
            dim: tuple(
                ((moon.position[dim], moon.velocity[dim]) for moon in moons)
            )
            for dim in Moon.dimensions
        }
        for k, v in state.items():
            if out.get(k, None):
                continue
            if v in states[k]:
                out[k] = steps
            else:
                states[k].add(v)
        for left, right in combinations(moons, 2):
            left ^ right
        _ = [moon.move() for moon in moons]
        steps += 1

    return out


state_space = search_space(moons)
print(reduce(lcm, state_space.values()))
