"""
You'll need to build a new emergency hull painting robot. The robot needs to be
able to move around on the grid of square panels on the side of your ship,
detect the color of its current panel, and paint its current panel black or
white. (All of the panels are currently black.)

The Intcode program will serve as the brain of the robot. The program uses
input instructions to access the robot's camera: provide 0 if the robot is over
a black panel or 1 if the robot is over a white panel. Then, the program will
output two values:

First, it will output a value indicating the color to paint the panel the robot
is over: 0 means to paint the panel black, and 1 means to paint the panel
white. Second, it will output a value indicating the direction the robot should
turn: 0 means it should turn left 90 degrees, and 1 means it should turn right
90 degrees. After the robot turns, it should always move forward exactly one
panel. The robot starts facing up.

The robot will continue running for a while like this and halt when it is
finished drawing. Do not restart the Intcode computer inside the robot during
this process.


"""
from pathlib import Path
from intcode import Computer
from collections import namedtuple
from typing import Dict, Tuple, List

Point = namedtuple("Point", ["x", "y"])


class HullRobot:
    turn_state: Dict[Tuple[str, int], str] = {
        ("north", 0): "west",
        ("north", 1): "east",
        ("west", 0): "south",
        ("west", 1): "north",
        ("south", 0): "east",
        ("south", 1): "west",
        ("east", 0): "north",
        ("east", 1): "south",
    }

    def __init__(self, input_ixs: List[str], start_color: int = 0):
        self.computer = Computer(input_ixs, [start_color])
        self.orientation = "north"
        self.loc = Point(0, 0)
        self.painted_points = {}

    def __call__(self):
        while not self.computer.is_halted:
            self.step()
        return self.painted_points

    def step(self):
        out = self.computer()
        self.paint(out)
        out = self.computer()
        self.turn(out)
        self.move()

    def paint(self, paint_value: int):
        self.painted_points[self.loc] = paint_value

    def turn(self, turn_value: int):
        self.orientation = self.turn_state[(self.orientation, turn_value)]

    def move(self):
        move_state = {
            "north": Point(self.loc.x, self.loc.y + 1),
            "south": Point(self.loc.x, self.loc.y - 1),
            "east": Point(self.loc.x + 1, self.loc.y),
            "west": Point(self.loc.x - 1, self.loc.y),
        }
        self.loc = move_state[self.orientation]

        self.computer << self.painted_points.get(self.loc, 0)


in_txt = Path("input11.txt").read_text().rstrip().split(",")

robot = HullRobot(in_txt)
painted_points = robot()
print(len(painted_points.keys()))

robot = HullRobot(in_txt, 1)
painted_points = robot()


def draw_points(painted_points: Dict[Point, int]):
    min_x = min(painted_points.keys(), key=lambda x: x.x).x
    max_x = max(painted_points.keys(), key=lambda x: x.x).x
    min_y = min(painted_points.keys(), key=lambda x: x.y).y
    max_y = max(painted_points.keys(), key=lambda x: x.y).y

    grid = [
        [" " for i in range(max_x - min_x + 1)]
        for i in range(max_y - min_y + 1)
    ]

    for point, color in painted_points.items():
        if color == 1:
            grid[abs(point.y)][point.x] = "#"
    for row in grid:
        print("".join(row))


draw_points(painted_points)
